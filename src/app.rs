use std::{env, io};

use crate::git_utils::Git;
use crate::mvn_utils::{reset_version, set_new_version};
use crate::{validation, CliArgs};

pub fn run(args: &CliArgs) -> io::Result<()> {
    validation::validate_args(&args)?;
    debug!("Received cli args = {:?}", &args);

    debug!("Checking git is installed...");
    validation::git_available()?;
    debug!("Git installed");

    debug!("Checking maven is installed...");
    validation::mvn_available()?;
    debug!("Maven installed");

    let path = env::current_dir()?;
    debug!("Project working dir = {:?}", &path);

    if let Some(ver) = &args.new_version {
        let git = Git::open(path.clone())?;
        git.new_branch(ver)?;
        set_new_version(ver, path.clone())?;
    } else if args.reset {
        reset_version(path.clone())?;
    }

    Ok(())
}
