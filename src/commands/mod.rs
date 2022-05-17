use clap::{Parser, Subcommand};

mod fly;
mod fly_config;

pub use fly::*;
pub use fly_config::*;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Used for fly related things
    #[clap(subcommand)]
    Fly(FlySubcommand),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct LsctlOptions {
    #[clap(subcommand)]
    pub command: Command,
}

pub trait CommandRunner {
    fn execute(&self) -> anyhow::Result<()>;
}
