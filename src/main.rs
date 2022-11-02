#[macro_use]
extern crate clap;

extern crate term;

mod cli;
mod errors;
mod terminal;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(c) = matches.value_of("cfg") {
        println!("Value for -c: {}", c);
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
