use clap::Subcommand;

use super::FlyConfigSubcommand;

#[derive(Subcommand, Debug)]
pub enum FlySubcommand {
    /// Used for updating, manipulating, or getting configs
    #[clap(subcommand)]
    Config(FlyConfigSubcommand),
}
