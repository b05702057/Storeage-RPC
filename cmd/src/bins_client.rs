use clap::{Arg, ArgMatches, Command, Parser};
use cmd::client_cmds::{app_commands, match_storage_cmds, print_result, repl};
use lab::lab2;
use tribbler::{
    config::{Config, DEFAULT_CONFIG_LOCATION},
    err::{TribResult, TribblerError},
    storage::{BinStorage, Storage},
};

#[derive(Parser, Debug)]
#[clap(name = "storage-client")]
struct Options {
    #[clap(short, long, default_value = DEFAULT_CONFIG_LOCATION)]
    config: String,
}

fn bin_cmd() -> [Command<'static>; 1] {
    [Command::new("bin").args(&[Arg::new("bin").required(true)])]
}

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> TribResult<()> {
    let args = Options::parse();
    let cfg = Config::read(Some(&args.config))?;
    let addrs = cfg.backs;
    let bc = lab2::new_bin_client(addrs).await?;
    let app = Command::new("bin-client")
        .subcommands(app_commands())
        .subcommands(bin_cmd());
    let mut client: Option<Box<dyn Storage>> = Some(bc.bin("").await?);
    println!("(now working on bin \"\")");

    loop {
        match repl(&app) {
            Ok(subcmd) => match match_cmds(&*bc, &mut client, subcmd.subcommand()).await {
                true => continue,
                false => break,
            },
            Err(_) => continue,
        }
    }
    Ok(())
}

pub async fn match_cmds(
    bin_client: &dyn BinStorage,
    client: &mut Option<Box<dyn Storage>>,
    subcmd: Option<(&str, &ArgMatches)>,
) -> bool {
    match subcmd {
        Some(("bin", v)) => match bin_client.bin(v.value_of("bin").unwrap()).await {
            Ok(binned) => {
                let _ = client.insert(binned);
                println!("(now working on bin \"{}\")", v.value_of("bin").unwrap());
                true
            }
            Err(e) => {
                // turbofish ::<String> just to pass a concrete type in order to compile
                print_result::<String>(Err(Box::new(TribblerError::Unknown(e.to_string()))));
                true
            }
        },
        Some(("exit", _)) => false,
        other => match client {
            Some(c) => match_storage_cmds(&**c, other).await,
            None => {
                println!("client must be initialized with the `bin` command first");
                true
            }
        },
    }
}
