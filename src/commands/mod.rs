use async_trait::async_trait;
use clap::{Parser, Subcommand};

mod fly;
mod fly_config;
mod js;

pub use fly::*;
pub use fly_config::*;
pub use js::*;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Used for fly related things
    #[clap(subcommand)]
    Fly(FlySubcommand),

    /// Used for js related things
    #[clap(subcommand)]
    Js(JsSubcommand),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct LsctlOptions {
    #[clap(subcommand)]
    pub command: Command,
}

#[async_trait]
pub trait CommandRunner {
    async fn execute(&self) -> anyhow::Result<()>;
}
