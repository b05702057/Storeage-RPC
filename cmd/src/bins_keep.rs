use clap::Parser;
use cmd::bins_run;
use log::LevelFilter;
use tribbler::config::DEFAULT_CONFIG_LOCATION;
use tribbler::err::TribResult;

/// starts a number of backend servers using a given bin config file
#[derive(Parser, Debug)]
#[clap(name = "bins-keep")]
struct Args {
    /// log level to use when starting the backends
    #[clap(short, long, default_value = "INFO")]
    log_level: LevelFilter,
    /// bin configuration file
    #[clap(short, long, default_value = DEFAULT_CONFIG_LOCATION)]
    config: String,
    /// addresses to send ready notifications to
    #[clap(short, long)]
    ready_addrs: Vec<String>,

    #[clap(long, default_value = "10")]
    recv_timeout: u64,
}

#[tokio::main]
async fn main() -> TribResult<()> {
    let pt = bins_run::ProcessType::Keep;
    let args = Args::parse();
    bins_run::main(
        pt,
        args.log_level,
        args.config,
        args.ready_addrs,
        args.recv_timeout,
    )
    .await
}
