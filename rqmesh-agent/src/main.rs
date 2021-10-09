use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = clap::App::new("rqmesh-agent")
        .arg(clap::Arg::with_name("STORE_LOCATION")
                .help("Location of sqlite database")
                .takes_value(true)
                .default_value("rqmesh-agent.db"))
        .arg(clap::Arg::with_name("CHECK_CMD")
                .long("check-cmd")
                .takes_value(true)
                .default_value("apk list sqlite --installed")
                .help("Command to validate base dependencies present"))
        .arg(clap::Arg::with_name("INSTALL_CMD")
                .long("install-cmd")
                .takes_value(true)
                .default_value("apk add sqlite"))
        .get_matches();

    eprintln!("{:?}", matches);
    println!("Hello, world!");
}
