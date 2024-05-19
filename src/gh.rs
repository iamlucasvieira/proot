use anyhow::{Context, Result};
use log::info;

use std::process::Command;

pub struct GhCli {
    base_command: String,
    list_args: Vec<String>
}

impl GhCli {
    pub fn new() -> Self {
        Self { 
            base_command: "gh".to_string(),
            list_args: vec![
                "pr".to_string(),
                "list".to_string(),
                "--json".to_string(),
                "id,number,title,url,state,isCrossRepository,baseRefName,headRefName,headRepositoryOwner".to_string()
            ]
        }
    }

    fn command(&self) -> Command {
        let cmd = Command::new(&self.base_command);
        cmd
    }


    fn run_command(&self, mut cmd: Command) -> Result<String> {
        let output = cmd.output().with_context(|| format!("Failed to run gh command"))?;

        if !output.status.success() {
            anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr));
        }

        String::from_utf8(output.stdout)
            .with_context(|| "Could not convert gh cli output to string")
    }

    pub fn check_health(&self) -> Result<String> {
        info!("Checking if gh cli is installed");
        let mut cmd = self.command();
        cmd.arg("--version");
        info!("Running command: {:?}", cmd);
        self.run_command(cmd)
    }

    pub fn open_pr_on_web(&self, pr_number: u64) -> Result<String> {
        info!("Opening PR on GitHub");
        let mut cmd = self.command();
        cmd.arg("pr")
            .arg("view")
            .arg(pr_number.to_string())
            .arg("--web");
        info!("Running command: {:?}", cmd);
        self.run_command(cmd)
    }

    pub fn get_all_prs(&self) -> Result<String> {
        info!("Getting PR list from GitHub");
        let mut cmd = self.command();
        cmd.args(&self.list_args);
        info!("Running command: {:?}", cmd);
        self.run_command(cmd)
    }

    pub fn get_filtered_prs(&self, filter: String) -> Result<String> {
        info!("Getting PR list from GitHub for user");
        let mut cmd = self.command();
        cmd.args(&self.list_args)
            .arg("--search".to_string())
            .arg(filter);
        info!("Running command: {:?}", cmd);
        self.run_command(cmd)
    }

}
