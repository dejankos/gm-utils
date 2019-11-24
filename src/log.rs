use std::fmt::Debug;

use colored::Colorize;

pub struct Logger {
    debug: bool,
    name: &'static str,
}

impl Logger {
    pub fn new(debug: bool, name: &'static str) -> Self {
        Logger { debug, name }
    }

    pub fn info(&self, msg: &str) {
        println!("{}", msg)
    }

    pub fn debug(&self, msg: &str) {
        if self.debug {
            println!("{}::{}", self.name.green(), msg.green())
        }
    }

    pub fn f_debug<T: Debug>(&self, msg: &str, t: &T) {
        if self.debug {
            self.debug(format!("{}::{} {:?}", self.name, msg, t).as_str())
        }
    }

    pub fn error<T: Debug>(&self, e: &T) {
        eprintln!("{}::{}", self.name.red(), format!("{:?}", e).red())
    }
}