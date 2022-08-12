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
extern crate clap;

use clap::{App, Arg};

const EXIT_OKAY: i32 = 0;
//const EXIT_ERROR: i32 = 1;

fn runtime() -> i32 {

    // Initialize the program
    let matches = App::new("Nova")
        .version("1.0")
        .author("Kris Nóva <kris@nivenly.com>")
        .about("My application.")
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .value_name("VERBOSE")
            .help("Toggle the verbosity bit.") // With <3 from @togglebit
            .takes_value(false))
        .get_matches();


    if let Some(v) = matches.value_of("verbose") {
        println!("Verbosity: {}", v);
    }

    return EXIT_OKAY;
}

fn main() {
    let exit_code = runtime();
    std::process::exit(exit_code);
}
