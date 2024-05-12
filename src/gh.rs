use anyhow::{Context, Result};
use std::process::Command;

// Run command: gh pr list --json "id,title,state,baseRefName,headRefName"
pub fn get_pr_string() -> Result<String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("list")
        .arg("--json")
        .arg("id,title,state,baseRefName,headRefName")
        .output()?;

    String::from_utf8(output.stdout).with_context(|| "Could not convert gh cli output to string")
}

pub fn check_health() -> Result<()> {
    let output = Command::new("gh").arg("--version").output()?;

    if !output.status.success() {
        anyhow::bail!("gh cli is not installed");
    }

    Ok(())
}
