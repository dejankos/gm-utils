use io::Result;
use std::io;
use std::io::{Error, ErrorKind};
use std::process::Command;

use crate::CliArgs;

const GIT: &str = "git";
const MVN: &str = "mvn";
const WHICH: &str = "which";

pub fn git_available() -> Result<()> {
    command_available(GIT)
}

pub fn mvn_available() -> Result<()> {
    command_available(MVN)
}

pub fn validate_args(args: &CliArgs) -> Result<()> {
    if !args.reset && args.new_version.is_none() {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                "No arguments provided!",
            )
        );
    }

    if args.reset && args.new_version.is_some() {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Can't reset and set new version {} at the same time", args.new_version.as_ref().unwrap()),
            )
        );
    }

    Ok(())
}

fn command_available(cmd: &str) -> Result<()> {
    Command::new(WHICH)
        .arg(cmd)
        .output()
        .map_err(|e| e)
        .and_then(|o|
            if o.stdout.is_empty() {
                Err(Error::new(ErrorKind::Other, err_msg(&cmd)))
            } else {
                Ok(())
            }
        )
}

fn err_msg(cmd: &str) -> String {
    format!("Command {} not found", cmd)
}