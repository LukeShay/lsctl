use clap::Parser;
use commands::*;

mod commands;
mod utils;

fn main() -> anyhow::Result<()> {
    let command = LsctlOptions::parse().command;

    match &command {
        Command::Fly(FlySubcommand::Config(FlyConfigSubcommand::New(options))) => options.execute(),
        Command::Fly(FlySubcommand::Config(FlyConfigSubcommand::Gen(options))) => options.execute(),
        Command::Fly(FlySubcommand::Config(FlyConfigSubcommand::Schema(options))) => {
            options.execute()
        }
        Command::Js(JsSubcommand::Build(options)) => options.execute(),
    }
}
