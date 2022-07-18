use std::{collections::HashSet, process::Command};

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use regex::Regex;

use crate::{
    models::fly_models::DeployConfig,
    utils::{collection_utils, command_utils},
};

use super::{FlyConfigGenOptions, FlyConfigSubcommand};

static FLYCTL: &str = "flyctl";

#[derive(Clone, Parser, Debug)]
pub struct FlyDeploy {
    /// The names of the input JSON config files
    #[clap(default_value = "vec![\"fly.json\"]")]
    pub input_files: Vec<String>,

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
            output_file: "fly.toml".to_string(),
            input_files: self.input_files.clone(),
        };

        fly_config_gen.execute().await.unwrap();

        let deploy_config = DeployConfig::new(&self.input_files)?;

        let deploy_config_hooks = deploy_config.hooks;

        let fly_apps_stdout = command_utils::stdout_or_bail2(
            Command::new(FLYCTL).arg("apps").arg("list"),
            "Failed to get Fly apps",
        )
        .unwrap();

        let should_launch = !Regex::new(&format!("{}\\s+", deploy_config.name))
            .unwrap()
            .is_match(&fly_apps_stdout);

        let fly_app_secrets = if !should_launch {
            command_utils::stdout_or_bail2(
                Command::new(FLYCTL)
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
                Command::new(FLYCTL)
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
                        Command::new(FLYCTL)
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
                            .arg(postgres.vm_size.to_string()),
                        "Failed to create Fly app database",
                    )
                    .unwrap();
                }

                if should_attach_postgres {
                    println!("Attaching the Postgres database");

                    command_utils::stream_stdout_or_bail(
                        Command::new(FLYCTL)
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

        if let Some(hooks) = deploy_config_hooks.clone() {
            if let Some(pre_deploy) = hooks.pre_deploy {
                println!("Running pre-deploy hook");

                let pre_deploy_vec = pre_deploy.split(" ").collect::<Vec<_>>();
                let (program, args) = pre_deploy_vec.split_at(1);

                command_utils::stream_stdout_or_bail(
                    Command::new(program[0]).args(args),
                    "Failed to run post-deploy hook",
                )
                .unwrap();
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
            Command::new(FLYCTL).args(args),
            "Failed to deploy the app",
        )
        .unwrap();

        if deploy_config.scaling.balance_method.is_static() {
            println!(
                "Updating app scaling to {}",
                deploy_config.scaling.min_count
            );

            command_utils::stream_stdout_or_bail(
                Command::new(FLYCTL)
                    .arg("scale")
                    .arg("count")
                    .arg(deploy_config.scaling.min_count.to_string())
                    .arg("--app")
                    .arg(&deploy_config.name),
                "Failed to set scaling on the app",
            )
            .unwrap();
        } else {
            println!(
                "Updating app autoscaling to method: {}, min: {}, max: {}",
                deploy_config.scaling.balance_method.to_string(),
                deploy_config.scaling.min_count,
                deploy_config.scaling.max_count,
            );

            command_utils::stream_stdout_or_bail(
                Command::new(FLYCTL)
                    .arg("autoscale")
                    .arg(deploy_config.scaling.balance_method.to_string())
                    .arg("--app")
                    .arg(&deploy_config.name)
                    .arg(&format!("min={}", deploy_config.scaling.min_count))
                    .arg(&format!("max={}", deploy_config.scaling.max_count)),
                "Failed to set autoscaling on the app",
            )
            .unwrap();
        }

        println!("Updating app memory to {}mb", deploy_config.scaling.memory);
        command_utils::stream_stdout_or_bail(
            Command::new(FLYCTL)
                .arg("scale")
                .arg("memory")
                .arg(deploy_config.scaling.memory.to_string())
                .arg("--app")
                .arg(&deploy_config.name),
            "Failed to set memory on the app",
        )
        .unwrap();

        let mut regions: HashSet<String> = HashSet::from_iter(deploy_config.regions);
        regions.insert(deploy_config.default_region);

        println!(
            "Updating app regions to {}",
            collection_utils::join_hash_set_of_strings(&regions, ", ")
        );

        command_utils::stdout_or_bail2(
            Command::new(FLYCTL)
                .arg("regions")
                .arg("set")
                .args(regions)
                .arg("--app")
                .arg(&deploy_config.name),
            "Failed to set regions on the app",
        )
        .unwrap();

        let regions: HashSet<String> = HashSet::from_iter(deploy_config.backup_regions);

        println!(
            "Updating app backup regions {}",
            collection_utils::join_hash_set_of_strings(&regions, ", ")
        );

        command_utils::stream_stdout_or_bail(
            Command::new(FLYCTL)
                .arg("regions")
                .arg("backup")
                .args(regions)
                .arg("--app")
                .arg(&deploy_config.name),
            "Failed to set backup regions on the app",
        )
        .unwrap();

        if let Some(hooks) = deploy_config_hooks.clone() {
            if let Some(post_deploy) = hooks.post_deploy {
                println!("Running post-deploy hook");

                let post_deploy_vec = post_deploy.split(" ").collect::<Vec<_>>();
                let (program, args) = post_deploy_vec.split_at(1);

                command_utils::stream_stdout_or_bail(
                    Command::new(program[0]).args(args),
                    "Failed to run post-deploy hook",
                )
                .unwrap();
            }
        }

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
