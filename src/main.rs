use clap::Parser;
use commands::{Command, CommandRunner, FlyConfigSubcommand, FlySubcommand, LsctlOptions};

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
    }
}
