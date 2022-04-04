use std::{
    process,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    time::Duration,
};

use lab::{lab1, lab2};
use log::{error, info, warn, LevelFilter};
use tokio::join;
use tribbler::{addr, config::Config, err::TribResult, storage::MemStorage};

#[derive(Debug, Clone)]
pub enum ProcessType {
    Back,
    Keep,
}

pub async fn main(
    t: ProcessType,
    log_level: LevelFilter,
    cfg: String,
    _ready_addrs: Vec<String>,
    recv_timeout: u64,
) -> TribResult<()> {
    env_logger::builder()
        .default_format()
        .filter_level(log_level)
        .init();
    let config = Arc::new(Config::read(Some(&cfg))?);

    println!("{:?}", config);
    let (tx, rdy) = mpsc::channel();

    let mut handles = vec![];
    let it = match t {
        ProcessType::Back => &config.backs,
        ProcessType::Keep => &config.keepers,
    };
    for (i, srv) in it.iter().enumerate() {
        if addr::check(srv)? {
            handles.push(tokio::spawn(run_srv(
                t.clone(),
                i,
                config.clone(),
                Some(tx.clone()),
            )));
        }
    }
    let proc_name = match t {
        ProcessType::Back => "backend",
        ProcessType::Keep => "keeper",
    };
    if handles.is_empty() {
        warn!("no {}s found for this host", proc_name);
        return Ok(());
    }
    info!("Waiting for ready signal from {}...", proc_name);
    match rdy.recv_timeout(Duration::from_secs(recv_timeout)) {
        Ok(msg) => match msg {
            true => info!("{}s should be ready to serve.", proc_name),
            false => {
                error!("{}s failed to start successfully", proc_name);
                process::exit(1);
            }
        },
        Err(_) => {
            error!("timed out waiting for {}s to start", proc_name);
            process::exit(1);
        }
    }
    for h in handles {
        match join!(h) {
            (Ok(_),) => (),
            (Err(e),) => {
                warn!("A {} failed to join: {}", proc_name, e);
            }
        };
    }
    Ok(())
}

#[allow(unused_must_use)]
async fn run_srv(t: ProcessType, idx: usize, config: Arc<Config>, tx: Option<Sender<bool>>) {
    match t {
        ProcessType::Back => {
            let cfg = config.back_config(idx, Box::new(MemStorage::default()), tx, None);
            info!("starting backend on {}", cfg.addr);
            lab1::serve_back(cfg).await;
        }
        ProcessType::Keep => {
            let cfg = config.keeper_config(idx, tx).unwrap();
            info!("starting keeper on {}", cfg.addr());
            lab2::serve_keeper(cfg).await;
        }
    };
}
