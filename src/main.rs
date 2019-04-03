use clap::{App, Arg, SubCommand};
use dialoguer::Confirmation;
use std::path::Path;
use std::process::exit;

pub mod subcommands;

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
        .subcommand(
            SubCommand::with_name("init").about("Initialize first configuration of MobTool"),
        )
        .arg(
            Arg::with_name("config-file")
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

    let subcommand = match cliopts.subcommand_name() {
        Some(subcommand) => subcommand,
        None => exit(0),
    };

    let verbosity_level: i32 = match cliopts.occurrences_of("verbosity") {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        _ => panic!("Verbosity level way beyond maximum (v: low, vv: medium, vvv: high)"),
    };

    let configuration_exists = cliopts.is_present("config-file");
    let first_execution = !Path::new("~/.mobtool").exists();

    if configuration_exists {
        let configuration_file = match cliopts.value_of("config-file") {
            Some(file_string) => Path::new(file_string),
            None => panic!("Non-existent path of configuration file"),
        };

        if !configuration_file.exists() {
            panic!("Given path of configuration file does not exist");
        }

        unimplemented!();
    } else if cliopts.is_present("init") {
        subcommands::init::start();
    } else {
        match cliopts.subcommand() {
            ("init", Some(sub_c)) => subcommands::init::start(),
            ("shell", Some(sub_c)) => subcommands::shell::start(sub_c.value_of("vm").unwrap()),
            ("startvm", Some(sub_c)) => subcommands::startvm::start(sub_c.value_of("vm").unwrap()),
            ("stopvm", Some(sub_c)) => subcommands::stopvm::start(sub_c.value_of("vm").unwrap()),
            _ => panic!("Non-existent subcommand received"),
        }
        if first_execution {
            println!("We have noticed that this is the first time you are executing MobTool.");
            if Confirmation::new()
                .with_text("Do you want to proceed to the creation of a configuration file?")
                .interact()
                .expect("Wrong option")
            {
                subcommands::init::start();
            } else {
                exit(0);
            }
        }
    }
}
