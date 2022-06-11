use std::{process::Output};

use anyhow::{bail, Result};

pub fn stdout_or_bail(output: Output, failure_message: &str) -> Result<String> {
    if !output.status.success() {
        bail!(
            "{}, {}",
            failure_message,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    match String::from_utf8(output.stdout) {
        Ok(plaintext) => Ok(plaintext),
        Err(e) => bail!(e),
    }
}
