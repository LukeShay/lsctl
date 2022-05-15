use std::collections::HashMap;

use crate::utils::file_utils;
use colored::*;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
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
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
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
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyBuild {
    builder: Option<String>,
    image: Option<String>,
    dockerfile: Option<String>,
    build_target: Option<String>,
    buildpacks: Option<Vec<String>>,
    args: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyDeploy {
    release_command: Option<String>,
    strategy: Option<FlyDeployStrategy>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyStatic {
    guest_path: String,
    url_prefix: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
enum FlyDeployStrategy {
    Canary,
    Rolling,
    BlueGreen,
    Immediate,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyGcpKms {
    project: String,
    key_ring: String,
    key: String,
    location: String,
    version: Option<u64>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyGcpSsm {
    project: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyDatabase {
    postgres: bool,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
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

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyServiceConcurrency {
    hard_limit: Option<u64>,
    soft_limit: Option<u64>,
    #[serde(alias = "type")]
    the_type: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
enum FlyServiceProtocol {
    TCP,
    UDP,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyServicePort {
    port: u64,
    force_https: Option<bool>,
    handlers: Vec<FlyServicePortHandler>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
enum FlyServicePortHandler {
    Http,
    Tls,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
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

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyServiceTcpCheck {
    interval: Option<u64>,
    grace_period: Option<String>,
    timeout: Option<u64>,
    restart_limit: Option<u64>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
enum FlyServiceHttpCheckProtocol {
    Http,
    Https,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
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

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyMount {
    source: String,
    destination: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
struct FlyExperimental {
    cmd: Option<Vec<String>>,
    entrypoint: Option<Vec<String>>,
}

pub fn config_new(arg_matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = arg_matches.value_of("name").unwrap();
    let organization = arg_matches.value_of("organization").unwrap();
    let database = arg_matches.is_present("database");
    let file_name = arg_matches.value_of("file").unwrap();

    println!("Creating new fly config file:");
    println!("    {:12} {}", "file".bold(), file_name);
    println!("    {:12} {}", "name".bold(), name);
    println!("    {:12} {}", "organization".bold(), organization);
    println!("    {:12} {}", "database".bold(), database);

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
        database: Some(FlyDatabase { postgres: database }),
        // metrics: None,
        services: Some(vec![FlyService {
            internal_port: 3000,
            processes: vec!["app".to_string()],
            protocol: Some(FlyServiceProtocol::TCP),
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

    return match file_utils::create_and_write_file(file_name, config_json) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

pub fn config_schema(arg_matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = arg_matches.value_of("file").unwrap();

    println!("Outputing fly config schema:");
    println!("    {} {}", "file".bold(), file_name);

    let schema = schema_for!(DeployConfig);

    return match file_utils::create_and_write_file(
        file_name,
        serde_json::to_string_pretty(&schema).unwrap(),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

pub fn config_gen(arg_matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = arg_matches.value_of("input-file").unwrap();
    let output_file = arg_matches.value_of("output-file").unwrap();

    println!("Generating fly config:");
    println!("    {} {}", "input file".bold(), input_file);
    println!("    {} {}", "output file".bold(), output_file);

    let contents = std::fs::read_to_string(input_file)?;

    let deploy_config: DeployConfig = serde_json::from_str(contents.as_str())?;

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
    };

    let toml_string = toml::to_string(&fly_config)?;

    return match file_utils::create_and_write_file(output_file, toml_string) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}
