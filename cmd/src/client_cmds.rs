use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use tribbler::{
    err::{TribResult, TribblerError},
    storage::{KeyValue, Pattern, Storage},
};

pub fn app_commands() -> [Command<'static>; 9] {
    let k = &[Arg::new("key").required(true)];
    let kv = &[
        Arg::new("key").required(true),
        Arg::new("value").required(true),
    ];
    let patt = &[
        Arg::new("prefix").required(false).default_value(""),
        Arg::new("suffix").required(false).default_value(""),
    ];
    let clk = &[Arg::new("clock").required(false).default_value("0")];
    [
        Command::new("get").args(k),
        Command::new("set").args(kv),
        Command::new("keys").args(patt),
        Command::new("list-get").args(k),
        Command::new("list-append").args(kv),
        Command::new("list-remove").args(kv),
        Command::new("list-keys").args(patt),
        Command::new("clock").args(clk),
        Command::new("exit"),
    ]
}

pub fn repl(app: &Command) -> TribResult<ArgMatches> {
    print!("> ");
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input.");

    let mut args = match shlex::split(&input) {
        Some(v) => v,
        None => {
            println!("error splitting args");
            return Err(Box::new(TribblerError::Unknown(
                "failed to split args".to_string(),
            )));
        }
    };
    let mut client_args = vec![app.get_name().to_string()];
    client_args.append(&mut args);
    let args = client_args;
    let matches = app.clone().try_get_matches_from(args);
    match matches {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("Failed to parse args: {}", e);
            Err(Box::new(TribblerError::Unknown(
                "failed to parse args".to_string(),
            )))
        }
    }
}

pub async fn match_storage_cmds(client: &dyn Storage, subcmd: Option<(&str, &ArgMatches)>) -> bool {
    match subcmd {
        Some(("get", v)) => print_result(client.get(v.value_of("key").unwrap()).await),
        Some(("set", v)) => {
            let kv = get_kv(v);
            print_result(client.set(&kv).await);
        }
        Some(("keys", v)) => {
            let pattern = get_pattern(v);
            print_result(client.keys(&pattern).await);
        }
        Some(("list-get", v)) => print_result(client.list_get(v.value_of("key").unwrap()).await),
        Some(("list-append", v)) => {
            let kv = get_kv(v);
            print_result(client.list_append(&kv).await);
        }
        Some(("list-remove", v)) => {
            let kv = get_kv(v);
            print_result(client.list_remove(&kv).await);
        }
        Some(("list-keys", v)) => {
            let pattern = get_pattern(v);
            print_result(client.list_keys(&pattern).await);
        }
        Some(("clock", v)) => match v.value_of("clock").unwrap().parse::<u64>() {
            Ok(clk) => print_result(client.clock(clk).await),
            Err(e) => println!("{:?}", e),
        },
        Some(("exit", _)) => return false,
        _ => println!("unexpected command. try again."),
    }
    true
}

fn get_kv(matches: &ArgMatches) -> KeyValue {
    KeyValue {
        key: matches.value_of("key").unwrap().to_string(),
        value: matches.value_of("value").unwrap().to_string(),
    }
}

fn get_pattern(matches: &ArgMatches) -> Pattern {
    Pattern {
        prefix: matches.value_of("prefix").unwrap().to_string(),
        suffix: matches.value_of("suffix").unwrap().to_string(),
    }
}

pub fn print_result<T: Debug>(x: Result<T, Box<dyn Error + Send + Sync>>) {
    println!("{:?}", x);
}
