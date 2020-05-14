#[macro_use]
extern crate log;
extern crate simplelog;

use std::process::exit;

use exitcode::{OK, USAGE};
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use structopt::StructOpt;

mod app;
mod file_utils;
mod git_utils;
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
    TermLogger::init(log_lvl(args.debug), Config::default(), TerminalMode::Mixed).unwrap();
    match app::run(&args) {
        Ok(_) => {
            info!("Done");
            exit(OK);
        }
        Err(e) => {
            error!("{}", &e);
            exit(USAGE);
        }
    }
}

fn log_lvl(debug: bool) -> LevelFilter {
    if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    }
}
