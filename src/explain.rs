use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use clap::Parser;
use color_eyre::eyre::eyre;
use color_eyre::{Report, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Explanation {
    conflicts: Vec<Lock>,
    examples: Vec<String>,
}

impl fmt::Display for Explanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Explanation {
            conflicts,
            examples,
        } = self;

        writeln!(f, "Conflicts with:")?;

        for conflict in conflicts {
            writeln!(f, "- {conflict}")?;
        }

        writeln!(f)?;
        writeln!(f, "Example queries acquiring this lock type:")?;

        for example in examples {
            writeln!(f, "- {example}")?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub enum Lock {
    AccessShareLock,
    RowShareLock,
    RowExclusiveLock,
    ShareUpdateExclusiveLock,
    ShareLock,
    ShareRowExclusiveLock,
    ExclusiveLock,
    AccessExclusiveLock,
}

impl Lock {
    fn explain(&self) -> Result<()> {
        let content = include_str!("../resources/lock-explanations.yaml");
        let explanations: HashMap<Lock, Explanation> = serde_yaml::from_str(content)?;

        let explanation = explanations
            .get(self)
            .ok_or_else(|| eyre!("failed to get explanation for {self:?}"))?;

        println!("{self:?}\n\n{explanation}");

        Ok(())
    }
}

impl fmt::Display for Lock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Lock::AccessShareLock => "AccessShareLock",
            Lock::RowShareLock => "RowShareLock",
            Lock::RowExclusiveLock => "RowExclusiveLock",
            Lock::ShareUpdateExclusiveLock => "ShareUpdateExclusiveLock",
            Lock::ShareLock => "ShareLock",
            Lock::ShareRowExclusiveLock => "ShareRowExclusiveLock",
            Lock::ExclusiveLock => "ExclusiveLock",
            Lock::AccessExclusiveLock => "AccessExclusiveLock",
        };

        write!(f, "{value}")
    }
}

impl FromStr for Lock {
    type Err = Report;

    fn from_str(value: &str) -> Result<Self> {
        let lock = match value {
            "AccessShareLock" => Lock::AccessShareLock,
            "RowShareLock" => Lock::RowShareLock,
            "RowExclusiveLock" => Lock::RowExclusiveLock,
            "ShareUpdateExclusiveLock" => Lock::ShareUpdateExclusiveLock,
            "ShareLock" => Lock::ShareLock,
            "ShareRowExclusiveLock" => Lock::ShareRowExclusiveLock,
            "ExclusiveLock" => Lock::ExclusiveLock,
            "AccessExclusiveLock" => Lock::AccessExclusiveLock,
            _ => return Err(eyre!("invalid lock type {value}")),
        };

        Ok(lock)
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    lock: Lock,
}

pub fn run(args: &Args) -> Result<()> {
    args.lock.explain()?;

    Ok(())
}
