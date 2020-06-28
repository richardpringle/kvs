use clap::{App, Arg, ArgMatches};
use kvs::{KvStore, Result};
use std::env;

fn main() -> Result<()> {
    let bin_name = env::var("CARGO_PKG_NAME").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let matches = App::new(bin_name.as_str())
        .version(version.as_str())
        .subcommand(App::new("get").arg(Arg::with_name("key").index(1).required(true)))
        .subcommand(
            App::new("set")
                .arg(Arg::with_name("key").index(1).required(true))
                .arg(Arg::with_name("value").index(2).required(true)),
        )
        .subcommand(
            App::new("remove")
                .alias("rm")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .get_matches();

    let store = KvStore::open("")?;

    match matches.subcommand_name() {
        Some("get") => get(store, matches.subcommand_matches("get").unwrap()),
        Some("set") => set(store, matches.subcommand_matches("set").unwrap()),
        Some("remove") => remove(store, matches.subcommand_matches("remove").unwrap()),
        _ => todo!(),
    }?
    .map(|value| println!("{}", value));

    Ok(())
}

fn get(mut store: KvStore, matches: &ArgMatches) -> Result<Option<String>> {
    let key = matches.value_of("key").unwrap().into();
    Ok(store.get(key)?.or_else(|| Some("Key not found".into())))
}

fn set(mut store: KvStore, matches: &ArgMatches) -> Result<Option<String>> {
    let key = matches.value_of("key").unwrap();
    let value = matches.value_of("value").unwrap();

    store.set(key.into(), value.into())?;

    Ok(None)
}

fn remove(mut store: KvStore, matches: &ArgMatches) -> Result<Option<String>> {
    let key = matches.value_of("key").unwrap().into();

    let _ = store.remove(key).map_err(|err| {
        println!("Key not found");
        err
    })?;

    Ok(None)
}
