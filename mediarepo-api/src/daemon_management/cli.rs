use crate::daemon_management::error::{DaemonError, DaemonResult};
use std::ffi::OsStr;
use tokio::process::{Child, Command};

#[derive(Debug)]
pub struct DaemonCli {
    daemon_path: String,
    repo_path: String,
    child: Option<Child>,
}

impl DaemonCli {
    pub fn new(daemon_path: String, repo_path: String) -> Self {
        Self {
            daemon_path,
            repo_path,
            child: None,
        }
    }

    /// Initializes a repository at the specified path
    #[tracing::instrument]
    pub async fn init_repo(&self) -> DaemonResult<()> {
        let output = self
            .run_command(vec!["--repo", self.repo_path.as_str(), "init"])
            .await?;
        tracing::debug!("Response: {}", String::from_utf8(output).unwrap());

        Ok(())
    }

    /// Starts a daemon for the given repository
    #[tracing::instrument]
    pub fn start_daemon(&mut self) -> DaemonResult<()> {
        let child = self.run_daemon_process(vec!["--repo", self.repo_path.as_str(), "start"])?;
        self.child = Some(child);

        Ok(())
    }

    /// Returns if the daemon is currently running
    pub fn daemon_running(&mut self) -> bool {
        if let Some(child) = &mut self.child {
            child.try_wait().map(|e| e.is_some()).unwrap_or(true)
        } else {
            false
        }
    }

    /// Returns the path the daemon is serving
    pub fn repo_path(&self) -> &String {
        &self.repo_path
    }

    /// Runs a daemon subcommand
    async fn run_command<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        &self,
        args: I,
    ) -> DaemonResult<Vec<u8>> {
        let child = self.run_daemon_process(args)?;
        let output = child.wait_with_output().await?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
            let stderr = String::from_utf8(output.stderr).map_err(|e| e.to_string())?;
            Err(DaemonError::from(format!("{}\n{}", stdout, stderr)))
        }
    }

    /// Runs a daemon process with the given args
    fn run_daemon_process<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        &self,
        args: I,
    ) -> DaemonResult<Child> {
        Command::new(&self.daemon_path)
            .args(args)
            .spawn()
            .map_err(DaemonError::from)
    }
}
