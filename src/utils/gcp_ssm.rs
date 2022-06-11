use std::process::Command;

use anyhow::Result;

use super::command_utils;

pub fn access_secret_version(project_id: &str, secret_name: &str, version: u16) -> Result<String> {
    let output = Command::new("gcloud")
        .arg("secrets")
        .arg("versions")
        .arg("access")
        .arg(format!("{}", version))
        .arg("--secret")
        .arg(secret_name)
        .arg("--project")
        .arg(project_id)
        .output()
        .unwrap();

    command_utils::stdout_or_bail(output, "Failed to access secret version")
}
