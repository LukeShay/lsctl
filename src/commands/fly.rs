use std::process::Command;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use regex::Regex;

use crate::{models::fly_models::DeployConfig, utils::command_utils};

use super::{FlyConfigGenOptions, FlyConfigSubcommand};

#[derive(Clone, Parser, Debug)]
pub struct FlyDeploy {
    /// The name of the input JSON config file
    #[clap(long, short, default_value = "fly.json")]
    pub input_file: String,

    /// The name of the output Fly toml file
    #[clap(long, short, default_value = "fly.toml")]
    pub output_file: String,

    /// The environment to generate the Fly config for
    #[clap(long, short, default_value = "dev")]
    pub environment: String,

    /// The image tag or ID to deploy
    #[clap(long)]
    pub image: Option<String>,

    /// Only perform builds locally using the local docker daemon
    #[clap(long)]
    pub local_only: bool,

    /// Perform builds on a remote builder instance instead of using the local docker daemon
    #[clap(long)]
    pub remote_only: bool,

    /// Do not use the build cache when building the image
    #[clap(long)]
    pub no_cache: bool,

    /// Return immediately instead of monitoring deployment progress
    #[clap(long)]
    pub detach: bool,
}

#[async_trait]
impl super::CommandRunner for FlyDeploy {
    async fn execute(&self) -> anyhow::Result<()> {
        let fly_config_gen = FlyConfigGenOptions {
            environment: self.environment.clone(),
            output_file: self.output_file.clone(),
            input_file: self.input_file.clone(),
        };

        fly_config_gen.execute().await.unwrap();

        let deploy_config = DeployConfig::new(&self.input_file, &self.output_file)?;

        let fly_apps_stdout = command_utils::stdout_or_bail2(
            Command::new("fly").arg("apps").arg("list"),
            "Failed to get Fly apps",
        )
        .unwrap();

        let should_launch = !Regex::new(&format!("{}\\s+", deploy_config.name))
            .unwrap()
            .is_match(&fly_apps_stdout);

        let fly_app_secrets = if !should_launch {
            command_utils::stdout_or_bail2(
                Command::new("fly")
                    .arg("secrets")
                    .arg("list")
                    .arg("--app")
                    .arg(deploy_config.name.as_str()),
                "Failed to get Fly app secrets",
            )
            .unwrap()
            .split("\n")
            .filter_map(|line| {
                let line = line.trim();
                let parts = line.split(" ").collect::<Vec<_>>();

                match parts.get(0) {
                    Some(&secret) => Some(secret.to_string()),
                    None => None,
                }
            })
            .collect::<Vec<_>>()
        } else {
            vec![]
        };

        if should_launch {
            println!("Launching new app");

            command_utils::stream_stdout_or_bail(
                Command::new("fly")
                    .arg("launch")
                    .arg("--no-deploy")
                    .arg("--copy-config")
                    .arg("--name")
                    .arg(deploy_config.name.as_str())
                    .arg("--org")
                    .arg(deploy_config.organization.as_str())
                    .arg("--region")
                    .arg(deploy_config.default_region.as_str()),
                "Failed to start Fly app",
            )
            .unwrap();
        }

        if let Some(database) = deploy_config.database {
            if let Some(postgres) = database.postgres {
                let postgres_name = format!("{}-postgres", deploy_config.name);
                let should_attach_postgres = !fly_app_secrets.contains(&"DATABASE_URL".to_string());
                let should_create_postgres = should_attach_postgres
                    && !Regex::new(&format!("{}\\s+", postgres_name))
                        .unwrap()
                        .is_match(&fly_apps_stdout);

                if should_create_postgres {
                    println!("Creating new Postgres database");

                    command_utils::stream_stdout_or_bail(
                        Command::new("fly")
                            .arg("postgres")
                            .arg("create")
                            .arg("--name")
                            .arg(&postgres_name)
                            .arg("--organization")
                            .arg(deploy_config.organization.as_str())
                            .arg("--region")
                            .arg(deploy_config.default_region.as_str())
                            .arg("--volume-size")
                            .arg(postgres.volume_size.to_string())
                            .arg("--initial-cluster-size")
                            .arg("2")
                            .arg("--vm-size")
                            .arg(postgres.vm_size.as_str()),
                        "Failed to create Fly app database",
                    )
                    .unwrap();
                }

                if should_attach_postgres {
                    println!("Attaching the Postgres database");

                    command_utils::stream_stdout_or_bail(
                        Command::new("fly")
                            .arg("postgres")
                            .arg("attach")
                            .arg("--postgres-app")
                            .arg(&postgres_name)
                            .arg("--app")
                            .arg(&deploy_config.name),
                        "Failed to attach Fly app database",
                    )
                    .unwrap();
                }
            }
        }

        let mut args = vec!["deploy", "--region", deploy_config.default_region.as_str()];

        if let Some(image) = &self.image {
            args.push("--image");
            args.push(image.as_str());
        }

        if self.local_only {
            args.push("--local-only");
        }

        if self.remote_only {
            args.push("--remote-only");
        }

        if self.no_cache {
            args.push("--no-cache");
        }

        if self.detach {
            args.push("--detach");
        }

        println!("Deploying the app");

        command_utils::stream_stdout_or_bail(
            Command::new("fly").args(args),
            "Failed to deploy the app",
        )
        .unwrap();

        anyhow::Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub enum FlySubcommand {
    /// Used for updating, manipulating, or getting configs
    #[clap(subcommand)]
    Config(FlyConfigSubcommand),
    /// Deploys a app to fly
    Deploy(FlyDeploy),
}
