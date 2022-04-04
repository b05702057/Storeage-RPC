use std::process;

use clap::Parser;
use tribbler::{
    addr,
    config::{self, DEFAULT_CONFIG_LOCATION},
    err::TribResult,
};

/// generates a [config::Config] based on the command arguments. The config
/// is written to the file specified by --file.
#[derive(Parser, Debug)]
#[clap(name = "bins-mkcfg")]
struct Options {
    /// set of IP addresses to use for the backends. Specify this flag multiple
    /// times to use more than one IP.
    #[clap(short, long, default_value = "localhost")]
    ip: Vec<String>,
    /// numbers of backends
    #[clap(short, long, default_value = "3")]
    backs: usize,
    /// number of keepers
    #[clap(short, long, default_value = "1")]
    keeps: usize,
    /// location to write the config file. Use `-` for stdout
    #[clap(long, default_value = DEFAULT_CONFIG_LOCATION)]
    file: String,
    /// whether or not to used fixed versus random port numbers
    #[clap(short, long)]
    fix: bool,
}

fn main() -> TribResult<()> {
    let args = Options::parse();

    if args.backs > 300 {
        eprintln!("too many backs: {}. must be <= 300", args.backs);
        process::exit(1)
    }
    if args.keeps > 10 {
        eprintln!("too many keepers: {}. Must be <= 10", args.keeps);
        process::exit(1)
    }

    let mut p = 3000;
    if !args.fix {
        p = addr::rand::rand_port();
    }

    let mut backs = vec![];
    let mut keepers = vec![];
    for i in 0..args.backs {
        backs.push(format!("{}:{}", args.ip[i % args.ip.len()], p));
        p += 1;
    }

    for i in 0..args.keeps {
        keepers.push(format!("{}:{}", args.ip[i % args.ip.len()], p));
        p += 1;
    }

    let cfg = config::Config { backs, keepers };

    cfg.write(Some(&args.file))
}
