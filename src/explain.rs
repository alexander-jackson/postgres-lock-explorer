use std::collections::HashMap;
use std::fmt;

use clap::Parser;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde::Deserialize;

use crate::lock::Lock;

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

#[derive(Debug, Parser)]
pub struct Args {
    lock: Lock,
}

pub fn run(args: Args) -> Result<()> {
    let lock = args.lock;

    let content = include_str!("../resources/lock-explanations.yaml");
    let explanations: HashMap<Lock, Explanation> = serde_yaml::from_str(content)?;

    let explanation = explanations
        .get(&args.lock)
        .ok_or_else(|| eyre!("failed to get explanation for {lock:?}"))?;

    println!("{lock:?}\n\n{explanation}");

    Ok(())
}
