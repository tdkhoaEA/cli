use clap::{App, Arg, SubCommand, AppSettings};


#[macro_use]
extern crate clap;

extern crate term;

mod cli;
mod errors;
mod terminal;

fn main() {

    let matches = cli::build_cli().get_matches();
    // let yaml = load_yaml!("cli.yml");
    // let matches = App::from_yaml(yaml).get_matches();

    if let Some(c) = matches.value_of("login") {
        println!("Logging in to Movey: {}", c);
    }

    // match matches.occurrences_of("login") {
    //     0 => println!("Nothing is awesome"),
    //     1 => println!("Some things are awesome"),
    //     2 => println!("Lots of things are awesome"),
    //     3 | _ => println!("EVERYTHING is awesome!"),
    // }
    if let Some(matches) = matches.subcommand_matches("add") {
        // Safe to use unwrap() because of the required() option
        println!("Adding file: {} {}", 
            matches.value_of("input").unwrap(), 
            matches.value_of("input1").unwrap()
        );
    }
    
    // You can check the presence of an argument
    if matches.is_present("out") {
        // Another way to check if an argument was present, or if it occurred multiple times is to
        // use occurrences_of() which returns 0 if an argument isn't found at runtime, or the
        // number of times that it occurred, if it was. To allow an argument to appear more than
        // once, you must use the .multiple(true) method, otherwise it will only return 1 or 0.
        if matches.occurrences_of("debug") > 2 {
            println!("Debug mode is REALLY on, don't be crazy");
        } else {
            println!("Debug mode kind of on");
        }
    }
    // Do your thing here
}
