/*===========================================================================*\
 *           MIT License Copyright (c) 2022 Kris Nóva <kris@nivenly.com>     *
 *                                                                           *
 *                ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓                *
 *                ┃   ███╗   ██╗ ██████╗ ██╗   ██╗ █████╗   ┃                *
 *                ┃   ████╗  ██║██╔═████╗██║   ██║██╔══██╗  ┃                *
 *                ┃   ██╔██╗ ██║██║██╔██║██║   ██║███████║  ┃                *
 *                ┃   ██║╚██╗██║████╔╝██║╚██╗ ██╔╝██╔══██║  ┃                *
 *                ┃   ██║ ╚████║╚██████╔╝ ╚████╔╝ ██║  ██║  ┃                *
 *                ┃   ╚═╝  ╚═══╝ ╚═════╝   ╚═══╝  ╚═╝  ╚═╝  ┃                *
 *                ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛                *
 *                                                                           *
 *                       This machine kills fascists.                        *
 *                                                                           *
\*===========================================================================*/

extern crate core;

use clap::*;
use log::*;
use simplelog::SharedLogger;
use syslog::*;

const EXIT_OKAY: i32 = 0;
//const EXIT_ERROR: i32 = 1;

fn runtime() -> i32 {
    // Initialize the rust environmental logger
    // which will respect the following env var levels:
    //
    // RUST_LOG=trace
    // RUST_LOG=info
    // RUST_LOG=debug
    // RUST_LOG=error
    // RUST_LOG=warn
    //simple_logger::init_with_env().unwrap();
    // TODO put this behind a flag

    let name = "nova";

    // Initialize the program
    let matches = App::new("Nova")
        .version("1.0")
        .author("Kris Nóva <kris@nivenly.com>")
        .about(name)
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .help("Toggle the verbosity bit.") // With <3 from @togglebit
                .takes_value(false),
        )
        .arg(
            Arg::with_name("logger")
                .short('l')
                .help("Set the logger.")
                .takes_value(true),
        )
        .get_matches();

    // The logger will log to stdout and the syslog by default.
    // We hold the opinion that the program is either "verbose"
    // or it's not.
    //
    // Normal mode: Info, Warn, Error
    // Verbose mode: Debug, Trace, Info, Warn, Error
    let logger_level = if matches.is_present("verbose") {
        log::Level::Trace
    } else {
        log::Level::Info
    };

    // Syslog formatter
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: name.into(),
        pid: 0,
    };

    // Initialize the logger
    let logger_simple =
        simplelog::SimpleLogger::new(logger_level.to_level_filter(), simplelog::Config::default());
    let logger_syslog = syslog::unix(formatter).unwrap();
    let _ = match multi_log::MultiLogger::init(
        vec![logger_simple, Box::new(BasicLogger::new(logger_syslog))],
        logger_level,
    ) {
        Ok(_) => {}
        Err(e) => panic!("unable to connect to syslog: {:?}", e),
    };

    // Initialize the program
    info!("Runtime initialized: {}", name);
    debug!("Runtime debugging enabled: {}", name);

    return EXIT_OKAY;
}

fn main() {
    let exit_code = runtime();
    std::process::exit(exit_code);
}
