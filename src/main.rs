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

use log::*;
use clap::*;
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

    // Initialize the program
    let matches = App::new("Nova")
        .version("1.0")
        .author("Kris Nóva <kris@nivenly.com>")
        .about("My application.")
        .arg(Arg::with_name("verbose")
            .short('v')
            .long("verbose")
            .help("Toggle the verbosity bit.") // With <3 from @togglebit
            .takes_value(false))
        .arg(Arg::with_name("logger")
            .short('l')
            .help("Set the logger.")
            .takes_value(true))
        .get_matches();

    // Simple logger
    let logger_simple = if matches.is_present("verbose") {
        // Trace
        simplelog::SimpleLogger::new(log::LevelFilter::Trace, simplelog::Config::default())
    }else {
        // Info
        simplelog::SimpleLogger::new(log::LevelFilter::Info, simplelog::Config::default())
    };

    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "nova".into(),
        pid: 0,
    };

    let logger_syslog = syslog::unix(formatter).unwrap();
    let _ = multi_log::MultiLogger::init(vec![logger_simple, Box::new(BasicLogger::new(logger_syslog))], log::Level::Info);

    // log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
    //     .map(|()| log::set_max_level(LevelFilter::Info));


    // if let Some(l) = matches.value_of("logger"){
    //     info!("Initializing logger: {}", l);
    //     match l {
    //         "stdout" => {},
    //         _ => {}
    //     }
    // }

    info!("Info logging...");
    warn!("Warn logging...");
    debug!("Debug logging...");
    trace!("Trace logging...");
    error!("Error logging...");

    return EXIT_OKAY;
}

fn main() {
    let exit_code = runtime();
    std::process::exit(exit_code);
}
