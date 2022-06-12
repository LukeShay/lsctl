use anyhow;
use handlebars::Handlebars;
use serde_json::json;
use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct DeployConfig {
    pub name: String,
    pub organization: String,
    pub default_region: String,
    pub gcp_kms: Option<FlyGcpKms>,
    pub gcp_ssm: Option<FlyGcpSsm>,
    pub database: Option<FlyDatabase>,
    pub kill_signal: Option<FlyKillSignal>,
    pub kill_timeout: Option<u64>,
    pub build: Option<FlyBuild>,
    pub deploy: Option<FlyDeploy>,
    pub statics: Option<Vec<FlyStatic>>,
    pub services: Option<Vec<FlyService>>,
    pub mounts: Option<Vec<FlyMount>>,
    pub environment: Option<HashMap<String, Vec<EnvironmentVariable>>>,
}

impl DeployConfig {
    pub fn new(file_path: &str, environment: &str) -> anyhow::Result<DeployConfig> {
        let contents = std::fs::read_to_string(file_path)?;

        let reg = Handlebars::new();

        let rendered_contents =
            reg.render_template(contents.as_str(), &json!({ "environment": environment }))?;

        match serde_json::from_str(rendered_contents.as_str()) {
            Ok(deploy_config) => Ok(deploy_config),
            Err(e) => anyhow::bail!(e),
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct EnvironmentVariable {
    pub key: String,
    #[serde(flatten)]
    pub value: EnvironmentVariableValue,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "snake_case"))]
pub enum EnvironmentVariableValue {
    Value(String),
    FromGcpKms { value: String },
    FromGcpSsm { name: String, version: u16 },
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyConfig {
    pub app: String,
    pub kill_signal: Option<FlyKillSignal>,
    pub kill_timeout: Option<u64>,
    pub build: Option<FlyBuild>,
    pub deploy: Option<FlyDeploy>,
    pub statics: Option<Vec<FlyStatic>>,
    pub services: Option<Vec<FlyService>>,
    pub mounts: Option<Vec<FlyMount>>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyBuild {
    pub builder: Option<String>,
    pub image: Option<String>,
    pub dockerfile: Option<String>,
    pub build_target: Option<String>,
    pub buildpacks: Option<Vec<String>>,
    pub args: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyDeploy {
    pub release_command: Option<String>,
    pub strategy: Option<FlyDeployStrategy>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyStatic {
    pub guest_path: String,
    pub url_prefix: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
pub enum FlyDeployStrategy {
    Canary,
    Rolling,
    Bluegreen,
    Immediate,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyGcpKms {
    pub project: String,
    pub key_ring: String,
    pub key: String,
    pub location: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyGcpSsm {
    pub project: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema, Default)]
pub struct FlyDatabase {
    pub postgres: Option<FlyDatabasePostgres>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyDatabasePostgres {
    pub cluster_size: u32,
    pub vm_size: String,
    pub volume_size: u32,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyService {
    pub internal_port: u64,
    pub processes: Vec<String>,
    pub concurrency: FlyServiceConcurrency,
    pub ports: Vec<FlyServicePort>,
    pub tcp_checks: Option<Vec<FlyServiceTcpCheck>>,
    pub http_checks: Option<Vec<FlyServiceHttpCheck>>,
    #[serde(skip_serializing)]
    pub protocol: Option<FlyServiceProtocol>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyServiceConcurrency {
    pub hard_limit: Option<u64>,
    pub soft_limit: Option<u64>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub the_type: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum FlyServiceProtocol {
    Tcp,
    Udp,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyServicePort {
    pub port: u64,
    pub force_https: Option<bool>,
    pub handlers: Vec<FlyServicePortHandler>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
pub enum FlyServicePortHandler {
    Http,
    Tls,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyServiceHttpCheck {
    pub interval: Option<u64>,
    pub grace_period: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub protocol: Option<FlyServiceHttpCheckProtocol>,
    pub timeout: Option<u64>,
    pub restart_limit: Option<u64>,
    pub tls_skip_verify: Option<bool>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyServiceTcpCheck {
    pub interval: Option<u64>,
    pub grace_period: Option<String>,
    pub timeout: Option<u64>,
    pub restart_limit: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "lowercase"))]
pub enum FlyServiceHttpCheckProtocol {
    Http,
    Https,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(deserialize = "camelCase", serialize = "UPPERCASE"))]
pub enum FlyKillSignal {
    SigInt,
    SigTerm,
    SigQuit,
    SigUsr1,
    SigUsr2,
    SigKill,
    SigStop,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyMount {
    pub source: String,
    pub destination: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyExperimental {
    pub cmd: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
}
