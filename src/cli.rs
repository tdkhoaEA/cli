use clap::{App, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("cli")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("cfg")
            .short("c")
            .takes_value(true))
        .subcommands(vec![])
}
