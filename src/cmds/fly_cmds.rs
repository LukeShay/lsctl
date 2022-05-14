use json5;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfig {
    name: String,
    organization: String,
    gcp_kms: FlyConfigGcpKms,
    gcp_ssm: FlyConfigGcpSsm,
    database: FlyConfigDatabase,
    metrics: FlyConfigMetrics,
    services: Vec<FlyConfigService>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigGcpKms {
    project: String,
    key_ring: String,
    key: String,
    location: String,
    version: Option<u16>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigGcpSsm {
    project: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigDatabase {
    postgres: bool,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigMetrics {
    port: u16,
    endpoint: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigService {
    internal_port: u16,
    processes: Vec<String>,
    concurrency: FlyConfigServiceConcurrency,
    ports: Vec<FlyConfigServicePort>,
    health_checks: Vec<FlyConfigServiceHealthCheck>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigServiceConcurrency {
    hard_limit: u16,
    soft_limit: u16,
    #[serde(alias = "type")]
    the_type: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigServicePort {
    handlers: Vec<FlyHandlers>,
    port: u16,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
enum FlyHandlers {
    Http,
    Tls,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FlyConfigServiceHealthCheck {
    interval: u16,
    grace_period: String,
    method: String,
    path: String,
    protocol: FlyProtocols,
    timeout: u16,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
enum FlyProtocols {
    Http,
    Https,
}

pub fn config_new(
    name: &str,
    organization: &str,
    database: bool,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "name: {}, organization: {}, database: {}, file_name: {}",
        name, organization, database, file_name
    );

    let config = FlyConfig {
        name: name.to_string(),
        organization: organization.to_string(),
        gcp_kms: FlyConfigGcpKms {
            project: "".to_string(),
            key_ring: "".to_string(),
            key: "".to_string(),
            location: "".to_string(),
            version: Some(1),
        },
        gcp_ssm: FlyConfigGcpSsm {
            project: "".to_string(),
        },
        database: FlyConfigDatabase { postgres: database },
        metrics: FlyConfigMetrics {
            port: 3000,
            endpoint: "/metrics".to_string(),
        },
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

    let config_json5 = json5::to_string(&config).unwrap();

    return match fs::write(format!("{}.json5", file_name), config_json5) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    };
}
