use std::{convert::{TryFrom, TryInto}, path::PathBuf};

mod initialization;

use clap::{App, Arg, ArgMatches};
use rqmesh_core::AgentInitializationContext;
use log::{LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn main() {
    let log_level = match dbg!(std::env::var("RUST_LOG_LEVEL")).and_then(|e| Ok(e.to_ascii_lowercase())).as_deref() {
        Ok("trace") => LevelFilter::Trace,
        Ok("debug") => LevelFilter::Debug,
        Ok("info") => LevelFilter::Info,
        Ok("error") => LevelFilter::Error,
        Ok("off") => LevelFilter::Off,
        Ok("warn") => LevelFilter::Warn,
        v => {
            eprintln!("Error setting logging level option {:?}, defaulting to Warn", v);
            LevelFilter::Warn
        }
    };

    TermLogger::init(log_level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Error initializing logging");

    let matches = clap::App::new("rqmesh-agent")
        .arg(clap::Arg::with_name("STORE_LOCATION")
                .help("Location of sqlite database")
                .takes_value(true)
                .multiple(false)
                .default_value("rqmesh-agent.db"))
        .arg(clap::Arg::with_name("CHECK_CMD")
                .long("check-cmd")
                .takes_value(true)
                .multiple(false)
                .default_value("apk list sqlite --installed")
                .help("Command to validate base dependencies present"))
        .arg(clap::Arg::with_name("INSTALL_CMD")
                .long("install-cmd")
                .takes_value(true)
                .multiple(false)
                .default_value("apk add sqlite"))
        .get_matches();
    
    let store_location = matches.value_of("STORE_LOCATION").expect("Must set STORE_LOCATION");
    let store_location : PathBuf = std::path::PathBuf::try_from(store_location).expect("STORE_LOCATION must be a valid path");
    let check_dependencies_command = matches.value_of("CHECK_CMD").expect("Must set a command to check for required dependencies");
    let install_dependcies_command = matches.value_of("INSTALL_CMD").expect("Must set a command to install dependencies");

    let init_context = AgentInitializationContext::new(store_location, check_dependencies_command, install_dependcies_command);

    let agent: Agent = init_context.try_into().expect("failed");
    
    println!("Hello, world!");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agent {

}