use clap::{App, Arg, ArgMatches};
// use kvs::KvStore;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
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

    match matches.subcommand_name() {
        Some("get") => get(matches.subcommand_matches("get").unwrap()),
        Some("set") => set(matches.subcommand_matches("set").unwrap()),
        Some("remove") => remove(matches.subcommand_matches("remove").unwrap()),
        _ => todo!(),
    };
}

fn get(matches: &ArgMatches) {
    let _key = matches.value_of("key").unwrap();
    eprintln!("unimplemented");
    std::process::exit(1);
}

fn set(matches: &ArgMatches) {
    let _key = matches.value_of("key").unwrap();
    let _value = matches.value_of("value").unwrap();
    eprintln!("unimplemented");
    std::process::exit(1);
}

fn remove(matches: &ArgMatches) {
    let _key = matches.value_of("key").unwrap();
    eprintln!("unimplemented");
    std::process::exit(1);
}
