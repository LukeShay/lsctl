use anyhow;
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use futures::executor::block_on;
use handlebars::Handlebars;
use serde_json::json;
use std::collections::HashMap;

use crate::utils::{file_utils, gcp_kms, gcp_ssm};
use colored::*;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct DeployConfig {
    name: String,
    organization: String,
    gcp_kms: Option<FlyGcpKms>,
    gcp_ssm: Option<FlyGcpSsm>,
    database: Option<FlyDatabase>,
    kill_signal: Option<FlyKillSignal>,
    kill_timeout: Option<u64>,
    build: Option<FlyBuild>,
    deploy: Option<FlyDeploy>,
    statics: Option<Vec<FlyStatic>>,
    services: Option<Vec<FlyService>>,
    mounts: Option<Vec<FlyMount>>,
    environment: Option<HashMap<String, Vec<EnvironmentVariable>>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct EnvironmentVariable {
    key: String,
    #[serde(flatten)]
    value: EnvironmentVariableValue,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "snake_case"))]
enum EnvironmentVariableValue {
    Value(String),
    FromGcpKms { value: String },
    FromGcpSsm { name: String, version: u16 },
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyConfig {
    name: String,
    organization: String,
    kill_signal: Option<FlyKillSignal>,
    kill_timeout: Option<u64>,
    build: Option<FlyBuild>,
    deploy: Option<FlyDeploy>,
    statics: Option<Vec<FlyStatic>>,
    services: Option<Vec<FlyService>>,
    mounts: Option<Vec<FlyMount>>,
    env: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyBuild {
    builder: Option<String>,
    image: Option<String>,
    dockerfile: Option<String>,
    build_target: Option<String>,
    buildpacks: Option<Vec<String>>,
    args: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyDeploy {
    release_command: Option<String>,
    strategy: Option<FlyDeployStrategy>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyStatic {
    guest_path: String,
    url_prefix: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
enum FlyDeployStrategy {
    Canary,
    Rolling,
    Bluegreen,
    Immediate,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyGcpKms {
    project: String,
    key_ring: String,
    key: String,
    location: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyGcpSsm {
    project: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyDatabase {
    postgres: Option<bool>,
    redis: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyService {
    internal_port: u64,
    processes: Vec<String>,
    concurrency: FlyServiceConcurrency,
    ports: Vec<FlyServicePort>,
    tcp_checks: Option<Vec<FlyServiceTcpCheck>>,
    http_checks: Option<Vec<FlyServiceHttpCheck>>,
    #[serde(skip_serializing)]
    protocol: Option<FlyServiceProtocol>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyServiceConcurrency {
    hard_limit: Option<u64>,
    soft_limit: Option<u64>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    the_type: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
enum FlyServiceProtocol {
    Tcp,
    Udp,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyServicePort {
    port: u64,
    force_https: Option<bool>,
    handlers: Vec<FlyServicePortHandler>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
enum FlyServicePortHandler {
    Http,
    Tls,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyServiceHttpCheck {
    interval: Option<u64>,
    grace_period: Option<String>,
    method: Option<String>,
    path: Option<String>,
    protocol: Option<FlyServiceHttpCheckProtocol>,
    timeout: Option<u64>,
    restart_limit: Option<u64>,
    tls_skip_verify: Option<bool>,
    headers: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyServiceTcpCheck {
    interval: Option<u64>,
    grace_period: Option<String>,
    timeout: Option<u64>,
    restart_limit: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
enum FlyServiceHttpCheckProtocol {
    Http,
    Https,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "UPPERCASE"))]
enum FlyKillSignal {
    SigInt,
    SigTerm,
    SigQuit,
    SigUsr1,
    SigUsr2,
    SigKill,
    SigStop,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyMount {
    source: String,
    destination: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyExperimental {
    cmd: Option<Vec<String>>,
    entrypoint: Option<Vec<String>>,
}

#[derive(Clone, Parser, Debug)]
pub struct FlyConfigNewOptions {
    /// The name of the fly app
    #[clap(long)]
    pub name: String,

    /// The organization of the fly app
    #[clap(long)]
    pub organization: String,

    /// Whether or not this app needs a database
    #[clap(long, default_value = "fly.json")]
    pub file: String,

    /// The name of the JSON config file
    #[clap(long)]
    pub database: bool,
}

#[async_trait]
impl super::CommandRunner for FlyConfigNewOptions {
    async fn execute(&self) -> anyhow::Result<()> {
        let file = &self.file;
        let name = &self.name;
        let organization = &self.organization;
        let database = &self.database;

        println!("Creating new fly config file:");
        println!("    {:12} {}", "file".bold(), file);
        println!("    {:12} {}", "name".bold(), name);
        println!("    {:12} {}", "organization".bold(), organization);
        println!("    {:12} {}", "database".bold(), database);

        let mut environment_map: HashMap<String, Vec<EnvironmentVariable>> = HashMap::new();

        environment_map.insert(
            "all".to_string(),
            vec![
                EnvironmentVariable {
                    key: "PLAINTEXT_VALUE".to_string(),
                    value: EnvironmentVariableValue::Value("plaintext value".to_string()),
                },
                EnvironmentVariable {
                    key: "FROM_GCP_KMS_VALUE".to_string(),
                    value: EnvironmentVariableValue::FromGcpKms {
                        value: "kms string".to_string(),
                    },
                },
                EnvironmentVariable {
                    key: "FROM_GCP_SSM_VALUE".to_string(),
                    value: EnvironmentVariableValue::FromGcpSsm {
                        name: "ssm name".to_string(),
                        version: 1,
                    },
                },
            ],
        );

        let config = DeployConfig {
            name: name.to_string(),
            organization: organization.to_string(),
            build: None,
            deploy: None,
            kill_signal: None,
            kill_timeout: None,
            mounts: None,
            statics: None,
            gcp_kms: None,
            gcp_ssm: None,
            database: Some(FlyDatabase {
                postgres: Some(*database),
                redis: None,
            }),
            environment: Some(environment_map),
            // metrics: None,
            services: Some(vec![FlyService {
                internal_port: 3000,
                processes: vec!["app".to_string()],
                protocol: Some(FlyServiceProtocol::Tcp),
                tcp_checks: None,
                concurrency: FlyServiceConcurrency {
                    hard_limit: Some(25),
                    soft_limit: Some(20),
                    the_type: "connections".to_string(),
                },
                ports: vec![
                    FlyServicePort {
                        handlers: vec![FlyServicePortHandler::Http],
                        port: 80,
                        force_https: None,
                    },
                    FlyServicePort {
                        handlers: vec![FlyServicePortHandler::Tls, FlyServicePortHandler::Http],
                        port: 443,
                        force_https: Some(true),
                    },
                ],
                http_checks: Some(vec![FlyServiceHttpCheck {
                    interval: Some(10000),
                    grace_period: Some("5s".to_string()),
                    method: Some("get".to_string()),
                    path: Some("/api/health".to_string()),
                    protocol: Some(FlyServiceHttpCheckProtocol::Http),
                    timeout: Some(2000),
                    headers: None,
                    restart_limit: None,
                    tls_skip_verify: None,
                }]),
            }]),
        };

        let config_json = serde_json::to_string_pretty(&config).unwrap();

        return match file_utils::create_and_write_file(file, config_json) {
            Ok(_) => Ok(()),
            Err(e) => anyhow::bail!("Error creating file: {}", e),
        };
    }
}

#[derive(Clone, Parser, Debug)]
pub struct FlyConfigGenOptions {
    /// The name of the input JSON config file
    #[clap(long, short, default_value = "fly.json")]
    pub input_file: String,

    /// The name of the output Fly toml file
    #[clap(long, short, default_value = "fly.toml")]
    pub output_file: String,

    /// The environment to generate the Fly config for
    #[clap(long, short, default_value = "dev")]
    pub environment: String,
}

#[async_trait]
impl super::CommandRunner for FlyConfigGenOptions {
    async fn execute(&self) -> anyhow::Result<()> {
        let input_file = &self.input_file;
        let output_file = &self.output_file;
        let environment = &self.environment;

        println!("Generating fly config:");
        println!("    {} {}", "input file".bold(), input_file);
        println!("    {} {}", "output file".bold(), output_file);

        let contents = std::fs::read_to_string(input_file)?;

        let reg = Handlebars::new();

        let rendered_contents = reg.render_template(
            contents.as_str(),
            &json!({"environment": &self.environment}),
        )?;

        let deploy_config: DeployConfig = serde_json::from_str(rendered_contents.as_str())?;

        let mut environment_map: HashMap<String, String> = HashMap::new();

        match &deploy_config.environment {
            Some(env) => {
                env.iter().for_each(|(k, v)| match k.as_str() {
                    "all" => {
                        let environment_all = insert_environment_variables(&deploy_config, v);
                        environment_map.extend(environment_all);
                    }
                    other if other == environment.as_str() => {
                        let environment_all = insert_environment_variables(&deploy_config, v);
                        environment_map.extend(environment_all);
                    }
                    _ => {}
                });
            }
            None => {}
        }

        let fly_config = FlyConfig {
            name: deploy_config.name,
            organization: deploy_config.organization,
            build: deploy_config.build,
            deploy: deploy_config.deploy,
            kill_signal: deploy_config.kill_signal,
            kill_timeout: deploy_config.kill_timeout,
            mounts: deploy_config.mounts,
            statics: deploy_config.statics,
            services: deploy_config.services,
            env: Some(environment_map),
        };

        let toml_string = toml::to_string(&fly_config)?;

        return match file_utils::create_and_write_file(output_file, toml_string) {
            Ok(_) => anyhow::Ok(()),
            Err(e) => anyhow::bail!("Error creating file: {}", e),
        };
    }
}

fn insert_environment_variables(
    deploy_config: &DeployConfig,
    v: &Vec<EnvironmentVariable>,
) -> HashMap<String, String> {
    let mut environment: HashMap<String, String> = HashMap::new();

    v.into_iter().for_each(|env_var| match &env_var.value {
        EnvironmentVariableValue::Value(value) => {
            environment.insert(String::from(env_var.key.as_str()), value.to_string());
        }
        EnvironmentVariableValue::FromGcpKms { value } => block_on(async {
            let gcp_kms_unwrapped = deploy_config
                .gcp_kms
                .as_ref()
                .expect("gcp_ssm config is not set");

            match gcp_kms::decrypt_ciphertext(
                gcp_kms_unwrapped.project.as_str(),
                gcp_kms_unwrapped.location.as_str(),
                gcp_kms_unwrapped.key_ring.as_str(),
                gcp_kms_unwrapped.key.as_str(),
                value.as_str(),
            ) {
                Ok(decrypted_value) => {
                    environment.insert(String::from(env_var.key.as_str()), decrypted_value);
                }
                Err(e) => {
                    println!("Error decrypting {}: {}", value, e);
                }
            }
        }),
        EnvironmentVariableValue::FromGcpSsm { name, version } => block_on(async {
            match gcp_ssm::access_secret_version(
                deploy_config
                    .gcp_ssm
                    .as_ref()
                    .expect("gcp_ssm config is not set")
                    .project
                    .as_str(),
                name.as_str(),
                *version,
            ) {
                Ok(secret_value) => {
                    environment.insert(String::from(env_var.key.as_str()), secret_value);
                }
                Err(e) => {
                    println!("Error accessing {}/{}: {}", name, version, e);
                }
            }
        }),
    });

    environment
}

#[derive(Clone, Parser, Debug)]
pub struct FlyConfigSchemaOptions {
    /// The name of the JSON config file
    #[clap(long, short)]
    pub file: Option<String>,
}

#[async_trait]
impl super::CommandRunner for FlyConfigSchemaOptions {
    async fn execute(&self) -> anyhow::Result<()> {
        let file = &self.file.as_deref().unwrap_or("schema.json");

        println!("Outputing fly config schema:");
        println!("    {} {}", "file".bold(), file);

        let schema = schema_for!(DeployConfig);

        return match file_utils::create_and_write_file(
            file,
            serde_json::to_string_pretty(&schema).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => anyhow::bail!("Error creating file: {}", e),
        };
    }
}

#[derive(Clone, Subcommand, Debug)]
pub enum FlyConfigSubcommand {
    /// Generates a new fly config file
    New(FlyConfigNewOptions),
    /// Generates the fly.toml file
    Gen(FlyConfigGenOptions),
    /// Generates the fly config schema
    Schema(FlyConfigSchemaOptions),
}
