use anyhow::Result;
use base64;
use std::{fs, path::Path, process::Command};

use super::command_utils;

pub fn decrypt_ciphertext(
    project_id: &str,
    location: &str,
    key_ring: &str,
    key: &str,
    ciphertext: &str,
) -> Result<String> {
    let ciphertext_base64 = base64::decode(ciphertext.to_string()).unwrap();

    fs::write("data.kms", ciphertext_base64).unwrap();

    let output = Command::new("gcloud")
        .arg("kms")
        .arg("decrypt")
        .arg("--project")
        .arg(project_id)
        .arg("--location")
        .arg(location)
        .arg("--keyring")
        .arg(key_ring)
        .arg("--key")
        .arg(key)
        .arg("--plaintext-file")
        .arg("-")
        .arg("--ciphertext-file")
        .arg("data.kms")
        .output()
        .unwrap();

    fs::remove_file(Path::new("data.kms")).unwrap();

    command_utils::stdout_or_bail(output, "Failed to decrypt ciphertext")
}
