use clap::{App, Arg, SubCommand};

fn main() {
    let cliopts = App::new("MobTool")
        .version("1.0.1")
        .author("Rui Lopes")
        .about("Mobility Networks Testing Tool")
        .subcommand(
            SubCommand::with_name("startvm")
                .about("Starts a specific vm")
                .arg(
                    Arg::with_name("vm")
                        .help("Name of the VM to be started")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("stopvm")
                .about("Stops a specific vm")
                .arg(
                    Arg::with_name("vm")
                        .help("Name of the VM to be stopped")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("shell")
                .about("Start a shell inside an already started vm")
                .arg(
                    Arg::with_name("vm")
                        .help("Name of the VM to create a shell session to")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Path to a TOML configuration file (default is ~/.mobtool)")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .required(false)
                .takes_value(false)
                .help("Verbosity level (v: low, vv: medium, vvv: high)"),
        )
        .get_matches();

    let verbosity_level: i32 = match cliopts.occurrences_of("verbosity") {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        _ => panic!("Verbosity level way beyond maximum (v: low, vv: medium, vvv: high)"),
    };


}
