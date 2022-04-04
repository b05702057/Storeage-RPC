use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use lab::{self, lab1};
use log::LevelFilter;
use tokio::{sync::mpsc::Sender as MpscSender, task::JoinHandle};

use tribbler::addr::rand::rand_port;
#[allow(unused_imports)]
use tribbler::{
    self,
    config::BackConfig,
    err::{TribResult, TribblerError},
    storage::{KeyList, KeyString, KeyValue, MemStorage, Pattern, Storage},
};

const DEFAULT_HOST: &str = "localhost:3000";

async fn setup(
    addr: Option<&str>,
    storage: Option<Box<dyn Storage + Send + Sync>>,
) -> TribResult<(Box<dyn Storage>, JoinHandle<TribResult<()>>, MpscSender<()>)> {
    // let _ = env_logger::builder()
    //     .default_format()
    //     .filter_level(LevelFilter::Info)
    //     .try_init();
    let addr = match addr {
        Some(x) => x,
        None => DEFAULT_HOST,
    };
    let storage = match storage {
        Some(x) => x,
        None => Box::new(MemStorage::new()),
    };
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
    let cfg = BackConfig {
        addr: addr.to_string(),
        storage: storage,
        ready: Some(tx.clone()),
        shutdown: Some(shut_rx),
    };

    let handle = spawn_back(cfg);
    let ready = rx.recv_timeout(Duration::from_secs(5))?;
    if !ready {
        return Err(Box::new(TribblerError::Unknown(
            "back failed to start".to_string(),
        )));
    }
    let client = lab1::new_client(format!("http://{}", addr).as_str()).await?;
    Ok((client, handle, shut_tx.clone()))
}

fn spawn_back(cfg: BackConfig) -> tokio::task::JoinHandle<TribResult<()>> {
    tokio::spawn(lab1::serve_back(cfg))
}

fn kv(key: &str, value: &str) -> KeyValue {
    KeyValue {
        key: key.to_string(),
        value: value.to_string(),
    }
}

fn pat(prefix: &str, suffix: &str) -> Pattern {
    Pattern {
        prefix: prefix.to_string(),
        suffix: suffix.to_string(),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_get_set() -> TribResult<()> {
    let (client, _handle, _tx) = setup(None, None).await?;
    assert_eq!(None, client.get("").await?);
    assert_eq!(None, client.get("hello").await?);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_get_set_overwrite() -> TribResult<()> {
    let (client, _handle, _tx) = setup(None, None).await?;
    client.set(&kv("h8liu", "run")).await?;
    assert_eq!(Some("run".to_string()), client.get("h8liu").await?);
    client.set(&kv("h8liu", "Run")).await?;
    assert_eq!(Some("Run".to_string()), client.get("h8liu").await?);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_set_none() -> TribResult<()> {
    let (client, _handle, _shut) = setup(None, None).await?;
    client.set(&kv("h8liu", "")).await?;
    assert_eq!(None, client.get("h8liu").await?);
    client.set(&kv("h8liu", "k")).await?;
    assert_eq!(Some("k".to_string()), client.get("h8liu").await?);
    client.set(&kv("h8he", "something")).await?;
    assert_eq!(Some("something".to_string()), client.get("h8he").await?);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_keys() -> TribResult<()> {
    let (client, _handle, _tx) = setup(None, None).await?;
    let _ = client.set(&kv("h8liu", "1")).await?;
    let _ = client.set(&kv("h8he", "2")).await?;
    let keys = client.keys(&pat("h8", "")).await?;
    let mut v = keys.0;
    v.sort();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], "h8he");
    assert_eq!(v[1], "h8liu");
    assert_eq!(0, client.list_get("lst").await?.0.len());
    Ok(())
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_list() -> TribResult<()> {
    let (client, _handle, _shut) = setup(None, None).await?;
    client.list_append(&kv("lst", "a")).await?;
    let l = client.list_get("lst").await?.0;
    assert_eq!(1, l.len());
    assert_eq!("a", l[0]);

    client.list_append(&kv("lst", "a")).await?;
    let l = client.list_get("lst").await?.0;
    assert_eq!(2, l.len());
    assert_eq!("a", l[0]);
    assert_eq!("a", l[1]);
    assert_eq!(2, client.list_remove(&kv("lst", "a")).await?);
    assert_eq!(0, client.list_get("lst").await?.0.len());

    client.list_append(&kv("lst", "h8liu")).await?;
    client.list_append(&kv("lst", "h7liu")).await?;
    let l = client.list_get("lst").await?.0;
    assert_eq!(2, l.len());
    assert_eq!("h8liu", l[0]);
    assert_eq!("h7liu", l[1]);

    let l = client.list_keys(&pat("ls", "st")).await?.0;
    assert_eq!(1, l.len());

    let l = client.list_keys(&pat("z", "")).await?.0;
    assert_eq!(0, l.len());

    let l = client.list_keys(&pat("", "")).await?.0;
    assert_eq!(1, l.len());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_list_keys() -> TribResult<()> {
    let (client, _srv, _shut) = setup(None, None).await?;
    let _ = client.list_append(&kv("t1", "v1")).await?;
    let _ = client.list_append(&kv("t2", "v2")).await?;
    let r = client.list_keys(&pat("", "")).await?.0;
    assert_eq!(2, r.len());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_awaited() -> TribResult<()> {
    let (_client, srv, _shut) = setup(None, None).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    srv.abort();
    let r = srv.await;
    assert!(r.is_err());
    assert!(r.unwrap_err().is_cancelled());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_bad_address() -> TribResult<()> {
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let cfg = BackConfig {
        addr: "^_^".to_string(),
        storage: Box::new(MemStorage::default()),
        ready: Some(tx),
        shutdown: None,
    };
    let handle = spawn_back(cfg);
    if let Ok(ready) = rx.recv_timeout(Duration::from_secs(1)) {
        if ready {
            panic!("server should not have sent true ready signal");
        }
    };
    let r = handle.await;
    assert!(r?.is_err());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_store_before_serve() -> TribResult<()> {
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let store = MemStorage::default();
    store.set(&kv("hello", "hi")).await?;
    let cfg = BackConfig {
        addr: DEFAULT_HOST.to_string(),
        storage: Box::new(store),
        ready: Some(tx),
        shutdown: None,
    };
    let _handle = spawn_back(cfg);
    let ready = rx.recv_timeout(Duration::from_secs(1))?;
    if !ready {
        panic!("failed to start")
    }
    let client = lab1::new_client(format!("http://{}", DEFAULT_HOST).as_str()).await?;
    assert_eq!(Some("hi".to_string()), client.get("hello").await?);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_multi_serve() -> TribResult<()> {
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let cfg = BackConfig {
        addr: DEFAULT_HOST.to_string(),
        storage: Box::new(MemStorage::default()),
        ready: Some(tx.clone()),
        shutdown: None,
    };
    let cfg2 = BackConfig {
        addr: "localhost:3001".to_string(),
        storage: Box::new(MemStorage::default()),
        ready: Some(tx.clone()),
        shutdown: None,
    };
    spawn_back(cfg);
    spawn_back(cfg2);
    let ready =
        rx.recv_timeout(Duration::from_secs(2))? && rx.recv_timeout(Duration::from_secs(2))?;
    if !ready {
        panic!("failed to start")
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_clock() -> TribResult<()> {
    let (client, _srv, _shut) = setup(None, None).await?;
    assert_eq!(2999, client.clock(2999).await?);
    assert_eq!(3000, client.clock(0).await?);
    assert_eq!(3001, client.clock(2999).await?);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_spawn_same_addr() -> TribResult<()> {
    let addr = DEFAULT_HOST.to_string();
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
    let cfg = BackConfig {
        addr: addr.clone(),
        storage: Box::new(MemStorage::default()),
        ready: Some(tx.clone()),
        shutdown: Some(shut_rx),
    };
    let handle = spawn_back(cfg);
    assert_eq!(true, rx.recv_timeout(Duration::from_secs(2))?);
    let _ = shut_tx.send(()).await;
    let _ = handle.await;
    thread::sleep(Duration::from_millis(500));
    let cfg = BackConfig {
        addr: DEFAULT_HOST.to_string(),
        storage: Box::new(MemStorage::default()),
        ready: Some(tx),
        shutdown: None,
    };
    let _ = spawn_back(cfg);
    assert_eq!(true, rx.recv_timeout(Duration::from_secs(2))?);

    let client = lab1::new_client(format!("http://{}", addr.clone()).as_str()).await?;
    client.set(&kv("hello", "hi")).await?;
    assert_eq!(Some("hi".to_string()), client.get("hello").await?);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_back_spawn_new_storage() -> TribResult<()> {
    let host = format!("localhost:{}", rand_port());
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
    let cfg = BackConfig {
        addr: host.clone(),
        storage: Box::new(MemStorage::default()),
        ready: Some(tx.clone()),
        shutdown: Some(shut_rx),
    };
    let handle = spawn_back(cfg);
    assert_eq!(true, rx.recv_timeout(Duration::from_secs(2))?);
    let client = lab1::new_client(format!("http://{}", host).as_mut()).await?;
    client.set(&kv("hello", "hi")).await?;
    let _ = shut_tx.send(()).await?;
    let _ = handle.await;
    tokio::time::sleep(Duration::from_millis(500)).await;
    let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
    let cfg = BackConfig {
        addr: host.clone(),
        storage: Box::new(MemStorage::default()),
        ready: Some(tx),
        shutdown: Some(shut_rx),
    };
    let _ = spawn_back(cfg);
    assert_eq!(true, rx.recv_timeout(Duration::from_secs(2))?);
    assert_eq!(None, client.get("hello").await?);
    let _ = shut_tx.send(()).await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_cli_ops() -> TribResult<()> {
    let (client, _srv, _shut) = setup(None, None).await?;
    let client = Arc::new(client);
    let mut handles = vec![];
    for _ in 0..5 {
        let addr = format!("http://{}", DEFAULT_HOST);
        let jh = tokio::spawn(async move {
            let client = match lab1::new_client(&addr).await {
                Ok(c) => c,
                Err(e) => return Err(TribblerError::Unknown(e.to_string())),
            };
            for _ in 0..10 {
                if let Err(e) = client.list_append(&kv("lst", "item")).await {
                    return Err(TribblerError::Unknown(e.to_string()));
                };
            }
            Ok(())
        });
        handles.push(jh);
    }
    for handle in handles {
        let res = handle.await;
        assert!(res.is_ok());
    }
    assert_eq!(50, client.list_get("lst").await?.0.len());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_shutdown() -> TribResult<()> {
    let (client, srv, shutdown) = setup(None, None).await?;
    assert!(client.set(&kv("hello", "hi")).await?);
    let _ = shutdown.send(()).await;
    let r = srv.await.unwrap();
    assert!(r.is_ok());
    match client.get("hello").await {
        Ok(v) => panic!(
            "uh oh..somehow the client still completed this request: {:?}",
            v
        ),
        Err(_) => (),
    };
    Ok(())
}
