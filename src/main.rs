mod cmds;
mod utils;

static UNREACHABLE: &str = "parser should ensure only valid subcommand names are used";

const FLY: &str = "fly";
const CONFIG: &str = "config";
const NEW: &str = "new";
const GEN: &str = "gen";
const SCHEMA: &str = "schema";

const FLY_JSON: &str = "fly.json";

fn main() {
    let cmd = clap::Command::new(env!("CARGO_CRATE_NAME"))
        .bin_name(env!("CARGO_CRATE_NAME"))
        .author("Luke Shay - https://lukeshay.com")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::command!(FLY)
                .about("Used for fly related things")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    clap::command!(CONFIG)
                        .about("Used for updating, manipulating, or getting configs")
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                        .subcommands(vec![
                            clap::command!(NEW)
                                .about("Generates a new fly config file")
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
                                        .default_value(FLY_JSON)
                                        .required(false),
                                ]),
                            clap::command!(GEN)
                                .about("Generates the fly.toml file")
                                .args(&[
                                    clap::arg!(--"input-file" <FILE_NAME>)
                                        .short('i')
                                        .help("The name of the input JSON config file")
                                        .allow_invalid_utf8(false)
                                        .default_value(FLY_JSON)
                                        .required(false),
                                    clap::arg!(--"output-file" <FILE_NAME>)
                                        .short('o')
                                        .help("The name of the output Fly toml file")
                                        .allow_invalid_utf8(false)
                                        .default_value("fly.toml")
                                        .required(false),
                                ]),
                            clap::command!(SCHEMA)
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

    let result = match cmd.get_matches().subcommand() {
        Some((FLY, fly_matches)) => match fly_matches.subcommand() {
            Some((CONFIG, fly_config_matches)) => match fly_config_matches.subcommand() {
                Some((NEW, fly_config_new_matches)) => {
                    cmds::fly_cmds::config_new(fly_config_new_matches)
                }
                Some((GEN, fly_config_gen_matches)) => {
                    cmds::fly_cmds::config_gen(fly_config_gen_matches)
                }
                Some((SCHEMA, fly_config_schema_matches)) => {
                    cmds::fly_cmds::config_schema(fly_config_schema_matches)
                }
                _ => unreachable!("{}", UNREACHABLE),
            },
            _ => unreachable!("{}", UNREACHABLE),
        },
        _ => unreachable!("{}", UNREACHABLE),
    };

    match result {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
