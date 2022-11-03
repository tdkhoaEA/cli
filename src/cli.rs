use clap::{App, Arg, SubCommand, AppSettings, ArgGroup};


pub fn build_cli() -> App<'static> {
    // let yaml = load_yaml!("cli.yml");
    // let matches = App::from_yaml(yaml).get_matches();
    // App::from_yaml(yaml);
    
    App::new("cli")
        // .version(crate_version!())
        // .author(crate_authors!())
        // .about(crate_description!())
        .arg(
            Arg::with_name("login")
                .short('l')
                .takes_value(true)
                .help("login to Movey") // Displayed when showing help info
                .long("movey-login")
                // specify this argument (Starts at 1)
        )   
        .arg(
            Arg::with_name("upload")
                .help("upload movey file") // Displayed when showing help info
                .short('u') // Trigger this arg with "-a"
                .long("upload") // Trigger this arg with "--awesome")
                // .multiple(true)
        )
        // .subcommand(
        //     SubCommand::with_name("add") // The name we call argument with
        //         .about("the <file> <file> to add") // The message displayed in "myapp -h"
        //         // or "myapp help"
        //         .version("0.1") // Subcommands can have independent version
        //         .author("Kevin K.") // And authors
                
        //         .arg(
        //             Arg::with_name("input") // And their own arguments
        //                 .index(1)
        //                 .required(true),
        //         )
        //         .arg(
        //             Arg::with_name("input1") // And their own arguments
                        
        //                 .index(2)
        //                 .required(true),
        //         )
        // )
        .args_from_usage(
            "--set-ver [ver] 'set version manually'
                                        --major         'auto inc major'
                                        --minor         'auto inc minor'
                                        --patch         'auto inc patch'",
        )
        // Create a group, make it required, and add the above arguments
        .group(
            ArgGroup::with_name("vers")
                .required(true)
                .args(&["ver", "major", "minor", "patch"]),
        )
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(Arg::from_usage("[INPUT_FILE] 'some regular input'").group("input"))
        .arg(Arg::from_usage("--spec-in [SPEC_IN] 'some special input argument'").group("input"))
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(
            Arg::with_name("config")
                .short('c')
                .takes_value(true)
                .requires("input"),
        )

        .setting(AppSettings::ColorAlways)
        .help_message("Print help information")
}
