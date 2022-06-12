use std::{
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
};

use anyhow::{bail, Result};

pub fn stdout_or_bail(output: Output, failure_message: &str) -> Result<String> {
    if !output.status.success() {
        bail!(
            "{}, {}, {}",
            failure_message,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    match String::from_utf8(output.stdout) {
        Ok(plaintext) => Ok(plaintext),
        Err(e) => bail!(e),
    }
}

pub fn stdout_or_bail2(command: &mut Command, failure_message: &str) -> Result<String> {
    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        bail!(
            "{}, {}, {}",
            failure_message,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    match String::from_utf8(output.stdout) {
        Ok(plaintext) => Ok(plaintext),
        Err(e) => bail!(e),
    }
}

pub fn stream_stdout_or_bail(command: &mut Command, failure_message: &str) -> Result<String> {
    let mut cmd = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }

    let output = cmd.wait_with_output()?;

    if !output.status.success() {
        bail!(
            "{}, {}, {}",
            failure_message,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    match String::from_utf8(output.stdout) {
        Ok(plaintext) => Ok(plaintext),
        Err(e) => bail!(e),
    }
}
