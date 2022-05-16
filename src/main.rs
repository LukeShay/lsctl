mod cmds;
mod utils;

static UNREACHABLE: &str = "parser should ensure only valid subcommand names are used";

fn main() {
    let cmd = clap::Command::new(env!("CARGO_CRATE_NAME"))
        .bin_name(env!("CARGO_CRATE_NAME"))
        .author("Luke Shay - https://lukeshay.com/")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::command!("fly")
                .about("Used for fly related things")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    clap::command!("config")
                        .about("Used for updating, manipulating, or getting configs")
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                        .subcommands(vec![
                            clap::command!("new")
                                .about("Generates a new config file")
                                .arg_required_else_help(true)
                                .args(&[
                                    clap::arg!(--"name" <NAME>)
                                        .help("The name of the fly app")
                                        .allow_invalid_utf8(false)
                                        .required(true),
                                    clap::arg!(--"organization" <ORGANIZATION>)
                                        .help("The organization of the fly app")
                                        .allow_invalid_utf8(false)
                                        .alias("org")
                                        .required(true),
                                    clap::arg!(--"database" <DATABASE>)
                                        .help("Whether or not this app needs a database")
                                        .takes_value(false)
                                        .required(false),
                                    clap::arg!(--"file" <FILE_NAME>)
                                        .short('o')
                                        .help("The name of the JSON config file")
                                        .allow_invalid_utf8(false)
                                        .default_value("fly.json")
                                        .required(false),
                                ]),
                            clap::command!("gen")
                                .about("Generates the fly.toml file")
                                .args(&[
                                    clap::arg!(--"input-file" <FILE_NAME>)
                                        .short('i')
                                        .help("The name of the input JSON config file")
                                        .allow_invalid_utf8(false)
                                        .default_value("fly.json")
                                        .required(false),
                                    clap::arg!(--"output-file" <FILE_NAME>)
                                        .short('o')
                                        .help("The name of the output Fly toml file")
                                        .allow_invalid_utf8(false)
                                        .default_value("fly.toml")
                                        .required(false),
                                ]),
                            clap::command!("schema")
                                .about("Generates the fly config schema")
                                .args(&[clap::arg!(--"file" <FILE_NAME>)
                                    .short('o')
                                    .help("The name of the JSON config file")
                                    .allow_invalid_utf8(false)
                                    .default_value("fly_schema.json")
                                    .required(false)]),
                        ]),
                ),
        );

    match cmd.get_matches().subcommand() {
        Some(("fly", fly_matches)) => match fly_matches.subcommand() {
            Some(("config", fly_config_matches)) => match fly_config_matches.subcommand() {
                Some(("new", fly_config_new_matches)) => {
                    match cmds::fly_cmds::config_new(fly_config_new_matches) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Some(("gen", fly_config_gen_matches)) => {
                    match cmds::fly_cmds::config_gen(fly_config_gen_matches) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Some(("schema", fly_config_schema_matches)) => {
                    match cmds::fly_cmds::config_schema(fly_config_schema_matches) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    }
                }
                _ => unreachable!("{}", UNREACHABLE),
            },
            _ => unreachable!("{}", UNREACHABLE),
        },
        _ => unreachable!("{}", UNREACHABLE),
    }
}
