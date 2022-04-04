#![allow(dead_code)]
//! module containing configuration functions which can aid in configuring
//! and running the tribbler service.

use std::fs;
use std::io::{stdout, Write};
use std::sync::mpsc::Sender;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Receiver;

use crate::err::TribResult;
use crate::storage::Storage;

pub const DEFAULT_CONFIG_LOCATION: &str = "bins.json";

/// a struct which represents the configuration for a particular storage backend
pub struct BackConfig {
    /// the address `<host>:<port>` combination to serve on
    pub addr: String,
    /// the storage object responsible for handling the storage API
    /// The inner type needs to be [Send] and [Sync] in order to use the
    /// storage within an `async` context.
    pub storage: Box<dyn Storage>,
    /// a channel which should a single message when the storage is ready to
    /// serve requests
    pub ready: Option<Sender<bool>>,
    pub shutdown: Option<Receiver<()>>,
}

use std::fmt::Debug;

impl Debug for BackConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackConfig")
            .field("addr", &self.addr)
            .field("ready", &self.ready)
            .field("shutdown", &self.shutdown)
            .finish()
    }
}

#[derive(Debug)]
/// Configuration representing a single keeper.
pub struct KeeperConfig {
    /// The addresses of back-ends
    pub backs: Vec<String>,
    /// The addresses of keepers
    pub addrs: Vec<String>,
    /// The index of this back-end
    pub this: usize,
    /// Non zero incarnation identifier
    pub id: u128,
    /// Send a value when the keeper is ready. The distributed key-value
    /// service should be ready to serve when *any* of the keepers is
    /// ready.
    pub ready: Option<Sender<bool>>,
}

impl KeeperConfig {
    pub fn addr(&self) -> &str {
        &self.addrs[self.this as usize]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// A config file defining the backend and keeper network addresses
pub struct Config {
    pub backs: Vec<String>,
    pub keepers: Vec<String>,
}

impl Config {
    fn location(l: Option<&str>) -> &str {
        l.unwrap_or(DEFAULT_CONFIG_LOCATION)
    }

    /// Reads from an optional path a tribbler configuration into a [Config]
    /// struct. If [None] is provided, [DEFAULT_CONFIG_LOCATION] is used.
    pub fn read(location: Option<&str>) -> TribResult<Config> {
        let file = Config::location(location);
        let pth = fs::canonicalize(file)?;
        Ok(serde_json::from_slice::<Config>(&fs::read(pth)?)?)
    }

    /// Writes a [Config] out to a file at a particular location. If [None] is
    /// specified, the location is [DEFAULT_CONFIG_LOCATION].
    ///
    /// If the specified location is `-`, then it will write to stdout
    pub fn write(&self, location: Option<&str>) -> TribResult<()> {
        let file = Config::location(location);
        let mut handle: Box<dyn Write> = match file {
            "-" => Box::new(stdout()),
            _ => {
                let pth = std::path::Path::new(file);
                let handle = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(pth)?;
                Box::new(handle)
            }
        };
        let mut contents = serde_json::to_vec_pretty(&self)?;
        contents.append(&mut "\n".as_bytes().to_vec());
        let _ = handle.write_all(&contents)?;
        Ok(())
    }

    /// gets the total number of backends in the config.
    pub fn back_count(&self) -> usize {
        self.backs.len()
    }

    /// gets the total number of keepers in the config.
    pub fn keeper_count(&self) -> usize {
        self.keepers.len()
    }

    /// build a [BackConfig] for the given index `i` in the list of backend
    /// addresses. `i` must be a valid index in the list of backends.
    ///
    /// You can choose to pass in a [Sender] where the receiving end of the
    /// channel can get a message when the backend using this configuration
    /// has started.
    pub fn back_config(
        &self,
        idx: usize,
        store: Box<dyn Storage + Send + Sync>,
        ready: Option<Sender<bool>>,
        shutdown: Option<Receiver<()>>,
    ) -> BackConfig {
        BackConfig {
            addr: self.backs[idx].to_string(),
            storage: store,
            ready,
            shutdown,
        }
    }

    /// build a [KeeperConfig] for the given index `i` in the list of keeper
    /// addresses. `i` must be a valid index into the list of keepers.
    ///
    /// You can choose to pass in a [Sender] where the receiving end of the
    /// channel can get a message when the backend using this configuration
    /// has started.
    pub fn keeper_config(&self, i: usize, tx: Option<Sender<bool>>) -> TribResult<KeeperConfig> {
        Ok(KeeperConfig {
            backs: self.backs.clone(),
            addrs: self.keepers.clone(),
            this: i,
            id: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos(),
            ready: tx,
        })
    }
}
