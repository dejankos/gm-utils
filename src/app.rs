use std::{env, io};

use crate::{CliArgs, validation};
use crate::git_utils::Git;
use crate::log::Logger;
use crate::mvn_utils::Mvn;


pub fn run(args: &CliArgs) -> io::Result<()> {
    let log = Logger::new(args.debug, "app");

    validation::validate_args(&args)?;
    log.f_debug("Received cli args = ", &args);

    log.debug("Checking git is installed...");
    validation::git_available()?;
    log.debug("Git installed.");

    log.debug("Checking maven is installed...");
    validation::mvn_available()?;
    log.debug("Maven installed.");

    let path = env::current_dir()?;
    log.f_debug("Project working dir =", &path);

    let mvn = Mvn::new(&args);
    if let Some(ver) = &args.new_version {
        let git = Git::open(path.clone(), &args)?;
        git.new_branch(ver)?;
        mvn.set_new_version(ver, path.clone())?;
    } else if args.reset {
        mvn.reset_version(path.clone())?;
    }

    Ok(())
}