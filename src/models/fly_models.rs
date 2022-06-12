use anyhow;
use serde_json::Value;
use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct DeployConfig {
    pub name: String,
    pub organization: String,
    pub default_region: String,

    #[serde(default)]
    pub regions: Vec<String>,

    #[serde(default)]
    pub backup_regions: Vec<String>,

    #[serde(default)]
    pub scaling: FlyScaling,

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
    pub environment: Option<Vec<EnvironmentVariable>>,
}

impl DeployConfig {
    pub fn new(file_paths: &Vec<String>) -> anyhow::Result<DeployConfig> {
        let json_files = file_paths
            .iter()
            .map(|file_path| {
                let contents = std::fs::read_to_string(file_path)
                    .expect(&format!("Failed to read file {}", file_path));

                match serde_json::from_str(&contents) {
                    Ok(deploy_config) => deploy_config,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<Value>>();

        let mut merged = json_files[0].clone();

        for json_file in json_files.iter().skip(1) {
            json_patch::merge(&mut merged, json_file);
        }

        match serde_json::from_value(merged) {
            Ok(deploy_config) => Ok(deploy_config),
            Err(e) => panic!("{}", e),
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyAutoscaling {
    #[serde(default = "fly_scaling_count_default")]
    pub min_count: u64,
    #[serde(default = "fly_scaling_count_default")]
    pub max_count: u64,
    #[serde(default = "FlyAutoscalingBalanceMethod::default")]
    pub balance_method: FlyAutoscalingBalanceMethod,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FlyAutoscalingBalanceMethod {
    Balanced,
    Standard,
    Static,
}

impl FlyAutoscalingBalanceMethod {
    pub fn default() -> FlyAutoscalingBalanceMethod {
        FlyAutoscalingBalanceMethod::Balanced
    }

    pub fn is_static(&self) -> bool {
        match self {
            FlyAutoscalingBalanceMethod::Static => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FlyAutoscalingBalanceMethod::Balanced => "balanced",
            FlyAutoscalingBalanceMethod::Standard => "standard",
            FlyAutoscalingBalanceMethod::Static => "static",
        }
        .to_string()
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct FlyScaling {
    #[serde(default = "fly_scaling_memory_default")]
    pub memory: u64,
    #[serde(default = "FlyVmSize::default")]
    pub vm_size: FlyVmSize,
    #[serde(default = "fly_scaling_count_default")]
    pub min_count: u64,
    #[serde(default = "fly_scaling_count_default")]
    pub max_count: u64,
    #[serde(default = "FlyAutoscalingBalanceMethod::default")]
    pub balance_method: FlyAutoscalingBalanceMethod,
}

impl Default for FlyScaling {
    fn default() -> Self {
        FlyScaling {
            min_count: fly_scaling_count_default(),
            max_count: fly_scaling_count_default(),
            memory: fly_scaling_memory_default(),
            vm_size: FlyVmSize::default(),
            balance_method: FlyAutoscalingBalanceMethod::default(),
        }
    }
}

fn fly_scaling_count_default() -> u64 {
    1
}

fn fly_scaling_memory_default() -> u64 {
    256
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub struct EnvironmentVariable {
    pub key: String,
    #[serde(flatten)]
    pub value: EnvironmentVariableValue,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    #[serde(default = "fly_database_postgres_cluster_size_default")]
    pub cluster_size: u64,
    #[serde(default = "FlyVmSize::default")]
    pub vm_size: FlyVmSize,
    #[serde(default = "fly_database_postgres_volume_size_default")]
    pub volume_size: u64,
}

fn fly_database_postgres_cluster_size_default() -> u64 {
    1
}

fn fly_database_postgres_volume_size_default() -> u64 {
    0
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, JsonSchema)]
pub enum FlyVmSize {
    #[serde(rename = "shared-cpu-1x")]
    SharedCpu1x,

    #[serde(rename = "dedicated-cpu-1x")]
    DedicatedCpu1x,

    #[serde(rename = "dedicated-cpu-2x")]
    DedicatedCpu2x,

    #[serde(rename = "dedicated-cpu-4x")]
    DedicatedCpu4x,

    #[serde(rename = "dedicated-cpu-8x")]
    DedicatedCpu8x,
}

impl FlyVmSize {
    pub fn default() -> Self {
        FlyVmSize::SharedCpu1x
    }

    pub fn to_string(&self) -> String {
        match self {
            FlyVmSize::SharedCpu1x => "shared-cpu-1x",
            FlyVmSize::DedicatedCpu1x => "dedicated-cpu-1x",
            FlyVmSize::DedicatedCpu2x => "dedicated-cpu-2x",
            FlyVmSize::DedicatedCpu4x => "dedicated-cpu-4x",
            FlyVmSize::DedicatedCpu8x => "dedicated-cpu-8x",
        }
        .to_string()
    }
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
    pub interval: Option<String>,
    pub grace_period: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub protocol: Option<FlyServiceHttpCheckProtocol>,
    pub timeout: Option<String>,
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
