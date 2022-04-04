//! function which runs a kv-server using the [serve_back] function
//! implementation
use clap::Parser;
use lab::lab1::serve_back;
use log::{info, LevelFilter};
use tribbler::{config::BackConfig, err::TribResult};

#[derive(Parser, Debug)]
#[clap(name = "kv-server")]
struct Options {
    #[clap(short, long, default_value = "127.0.0.1:7799")]
    address: String,

    #[clap(short, long, default_value = "INFO")]
    log_level: LevelFilter,
}

#[tokio::main]
async fn main() -> TribResult<()> {
    let options = Options::parse();
    env_logger::builder()
        .default_format()
        .filter_level(options.log_level)
        .init();
    let memstorage = Box::new(tribbler::storage::MemStorage::new());
    let addr = options.address.clone();
    let config = BackConfig {
        addr: options.address,
        storage: memstorage,
        ready: None,
        shutdown: None,
    };
    let x = serve_back(config);
    info!("============================================");
    info!("KV SERVING AT ::: http://{}", &addr,);
    info!("============================================");
    x.await
}
