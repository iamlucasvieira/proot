use anyhow::{Context, Result};
use log::{error, info};

use std::process::Command;
// Run command: gh pr list --json "id,title,state,baseRefName,headRefName"
pub fn get_pr_string() -> Result<String> {
    info!("Getting PR list from GitHub");
    let output = Command::new("gh")
        .arg("pr")
        .arg("list")
        .arg("--json")
        .arg("id,number,title,url,state,isCrossRepository,baseRefName,headRefName,headRepositoryOwner")
        .output()?;

    String::from_utf8(output.stdout).with_context(|| "Could not convert gh cli output to string")
}

pub fn check_health() -> Result<()> {
    info!("Checking if gh cli is installed");
    let output = Command::new("gh").arg("--version").output()?;

    if !output.status.success() {
        error!("gh cli is not installed");
        anyhow::bail!("gh cli is not installed");
    }

    Ok(())
}
