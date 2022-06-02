use clap::Parser;
use commands::*;

mod commands;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = LsctlOptions::parse().command;

    match &command {
        Command::Fly(FlySubcommand::Config(FlyConfigSubcommand::New(options))) => {
            options.execute().await
        }
        Command::Fly(FlySubcommand::Config(FlyConfigSubcommand::Gen(options))) => {
            options.execute().await
        }
        Command::Fly(FlySubcommand::Config(FlyConfigSubcommand::Schema(options))) => {
            options.execute().await
        }
        Command::Js(JsSubcommand::Build(options)) => options.execute().await,
    }
}
