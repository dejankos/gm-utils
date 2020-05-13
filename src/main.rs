use std::process::exit;

use exitcode::{OK, USAGE};
use structopt::StructOpt;

use crate::log::Logger;

mod app;
mod file_utils;
mod git_utils;
mod log;
mod mvn_utils;
mod validation;

#[derive(StructOpt, Debug)]
pub struct CliArgs {
    #[structopt(help = "Git branch / mvn project version")]
    new_version: Option<String>,
    #[structopt(short, long, help = "Activate debug mode")]
    debug: bool,
    #[structopt(short, long, help = "Reset maven project version")]
    reset: bool,
}

fn main() {
    let args = CliArgs::from_args();
    let log = Logger::new(args.debug, "main");
    match app::run(&args) {
        Ok(_) => {
            log.info("Done");
            exit(OK);
        }
        Err(e) => {
            log.error(&e);
            exit(USAGE);
        }
    }
}
