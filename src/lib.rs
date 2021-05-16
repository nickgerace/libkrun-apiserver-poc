use anyhow::{anyhow, Result};
use log::{error, info};
use std::{
    path::Path,
    process::{Command, Stdio},
};

pub fn krunvm(args: &Vec<String>) -> Result<Option<i32>> {
    info!("Executing krunvm with args: {:?}", args);
    let output = Command::new("krunvm")
        .args(args.iter())
        .env("LD_LIBRARY_PATH", "/usr/local/lib64")
        .output()?;

    // Use a boolean to avoid cloning "output.stderr".
    let is_stderr_empty = &output.stderr.is_empty();
    for line in String::from_utf8(output.stderr)?.lines() {
        error!(">>>> {}", line);
    }
    for line in String::from_utf8(output.stdout)?.lines() {
        info!(">>>> {}", line);
    }
    match is_stderr_empty {
        false => Err(anyhow!(
            "Encountered error(s) during krunvm command execution"
        )),
        true => Ok(output.status.code()),
    }
}

pub fn validate() -> Result<()> {
    let so_files: Vec<&str> = vec![
        "/usr/local/lib64/libkrun.so",
        "/usr/local/lib64/libkrunfw.so",
    ]
    .iter()
    .filter(|&path| !Path::new(path).exists())
    .cloned()
    .collect();
    if !so_files.is_empty() {
        return Err(anyhow!("Shared object files not found: {:?}", so_files));
    }

    Command::new("krunvm")
        .stdout(Stdio::null())
        .arg("--help")
        .env("LD_LIBRARY_PATH", "/usr/local/lib64")
        .spawn()?;
    Ok(())
}
