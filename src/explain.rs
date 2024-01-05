use std::str::FromStr;

use clap::Parser;
use color_eyre::eyre::eyre;
use color_eyre::{Report, Result};

#[derive(Copy, Clone, Debug)]
pub enum Lock {
    ShareLock,
}

impl Lock {
    fn explain(&self) {
        match self {
            Lock::ShareLock => println!("Share locks protect a table from concurrent data changes. This will block insertions and updates, but allow selects to continue as normal."),
        }
    }
}

impl FromStr for Lock {
    type Err = Report;

    fn from_str(value: &str) -> Result<Self> {
        let lock = match value {
            "ShareLock" => Lock::ShareLock,
            _ => return Err(eyre!("invalid lock type {value}")),
        };

        Ok(lock)
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    lock: Lock,
}

pub fn run(args: &Args) {
    args.lock.explain();
}
