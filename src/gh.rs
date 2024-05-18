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

    if !output.status.success() {
        error!("Failed to get PR list from GitHub");
        anyhow::bail!("Failed to get PR list from GitHub");
    }

    String::from_utf8(output.stdout).with_context(|| "Could not convert gh cli output to string")
}

pub fn open_pr_on_web(pr_number: u64) -> Result<()> {
    info!("Opening PR on GitHub");
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number.to_string())
        .arg("--web")
        .output()?;

    if !output.status.success() {
        check_health()?;
        anyhow::bail!(
            "Failed to open PR on GitHub: {}. It may not exist.",
            pr_number
        );
    }

    Ok(())
}

pub fn check_health() -> Result<()> {
    info!("Checking if gh cli is installed");
    let output = Command::new("gh").arg("--version").output()?;

    if !output.status.success() {
        anyhow::bail!("gh cli is not installed");
    }

    Ok(())
}
