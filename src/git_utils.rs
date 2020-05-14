use io::Result;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, Output};

const ERR_MSG: &str = "Error executing git command";

pub struct Git {
    repository: Repository,
}

pub struct Repository {
    project_path: PathBuf,
}

trait OutputHandler {
    fn handle_git_cmd(self) -> Result<String>;
}

impl OutputHandler for Output {
    fn handle_git_cmd(self) -> Result<String> {
        if !self.status.success() && !self.stderr.is_empty() {
            let err_msg = match String::from_utf8(self.stderr) {
                Ok(msg) => msg,
                Err(e) => e.to_string(),
            };

            Err(Error::new(ErrorKind::InvalidData, err_msg))
        } else {
            Ok(String::from_utf8(self.stdout).unwrap())
        }
    }
}

impl Git {
    pub fn open(project_path: PathBuf) -> Result<Self> {
        let repo = Repository::open(project_path)?;
        Ok(Git { repository: repo })
    }

    pub fn new_branch(&self, b_name: &str) -> Result<()> {
        self.repository.branch_exists(b_name)?;
        let current_branch = self.repository.current_branch()?;
        info!("Creating new git branch {} from {}", b_name, current_branch);

        self.repository.new_branch(b_name, current_branch.trim())?;
        info!(
            "Branch {} successfully created from {}",
            b_name, current_branch,
        );

        Ok(())
    }
}

impl Repository {
    fn open(project_path: PathBuf) -> Result<Self> {
        Command::new("git")
            .args(&["-C", project_path.to_str().unwrap(), "rev-parse"])
            .output()
            .expect(ERR_MSG)
            .handle_git_cmd()?;

        Ok(Repository { project_path })
    }

    fn new_branch(&self, b_name: &str, current_branch: &str) -> Result<String> {
        Command::new("git")
            .args(&[
                "-C",
                self.project_path.to_str().unwrap(),
                "checkout",
                "-b",
                b_name,
                current_branch,
            ])
            .output()
            .expect(ERR_MSG)
            .handle_git_cmd()
    }

    fn current_branch(&self) -> Result<String> {
        Command::new("git")
            .args(&[
                "-C",
                self.project_path.to_str().unwrap(),
                "rev-parse",
                "--abbrev-ref",
                "HEAD",
            ])
            .output()
            .expect(ERR_MSG)
            .handle_git_cmd()
    }

    fn branch_exists(&self, b_name: &str) -> Result<String> {
        let res = Command::new("git")
            .args(&[
                "-C",
                self.project_path.to_str().unwrap(),
                "show-ref",
                format!("refs/heads/{}", b_name).as_str(),
            ])
            .output()
            .expect(ERR_MSG)
            .handle_git_cmd()?;

        if !res.is_empty() {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("Branch {} already exists!", b_name),
            ))
        } else {
            Ok(res)
        }
    }
}
