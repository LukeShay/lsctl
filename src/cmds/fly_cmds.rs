use crate::utils::file_utils;
use colored::*;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfig {
    name: String,
    organization: String,
    #[serde(skip_serializing)]
    gcp_kms: Option<FlyConfigGcpKms>,
    #[serde(skip_serializing)]
    gcp_ssm: Option<FlyConfigGcpSsm>,
    #[serde(skip_serializing)]
    database: Option<FlyConfigDatabase>,
    metrics: Option<FlyConfigMetrics>,
    services: Vec<FlyConfigService>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigGcpKms {
    project: String,
    key_ring: String,
    key: String,
    location: String,
    version: Option<u16>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigGcpSsm {
    project: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigDatabase {
    postgres: bool,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigMetrics {
    port: u16,
    endpoint: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigService {
    internal_port: u16,
    processes: Vec<String>,
    concurrency: FlyConfigServiceConcurrency,
    ports: Vec<FlyConfigServicePort>,
    health_checks: Vec<FlyConfigServiceHealthCheck>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigServiceConcurrency {
    hard_limit: u16,
    soft_limit: u16,
    #[serde(alias = "type")]
    the_type: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigServicePort {
    handlers: Vec<FlyHandlers>,
    port: u16,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
enum FlyHandlers {
    Http,
    Tls,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FlyConfigServiceHealthCheck {
    interval: u16,
    grace_period: String,
    method: String,
    path: String,
    protocol: FlyProtocols,
    timeout: u16,
}

#[derive(Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
enum FlyProtocols {
    Http,
    Https,
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

    let config = FlyConfig {
        name: name.to_string(),
        organization: organization.to_string(),
        gcp_kms: None,
        gcp_ssm: None,
        database: Some(FlyConfigDatabase { postgres: database }),
        metrics: None,
        services: vec![FlyConfigService {
            internal_port: 3000,
            processes: vec!["app".to_string()],
            concurrency: FlyConfigServiceConcurrency {
                hard_limit: 25,
                soft_limit: 20,
                the_type: "connections".to_string(),
            },
            ports: vec![
                FlyConfigServicePort {
                    handlers: vec![FlyHandlers::Http],
                    port: 80,
                },
                FlyConfigServicePort {
                    handlers: vec![FlyHandlers::Tls, FlyHandlers::Http],
                    port: 443,
                },
            ],
            health_checks: vec![FlyConfigServiceHealthCheck {
                interval: 10000,
                grace_period: "5s".to_string(),
                method: "get".to_string(),
                path: "/api/health".to_string(),
                protocol: FlyProtocols::Http,
                timeout: 2000,
            }],
        }],
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

    let schema = schema_for!(FlyConfig);

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

    let config: FlyConfig = serde_json::from_str(contents.as_str())?;

    let toml_string = toml::to_string(&config)?;

    return match file_utils::create_and_write_file(output_file, toml_string) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}
