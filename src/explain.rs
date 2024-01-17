use std::collections::HashMap;

use clap::Parser;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde::Deserialize;
use serde::Serialize;
use tera::Context;
use tera::Tera;

use crate::lock::Lock;

const TEMPLATE_NAME: &str = "lock-explanation";

#[derive(Serialize)]
struct TemplateContext {
    lock: Lock,
    conflicts: Vec<Lock>,
    examples: Vec<String>,
    blocked_examples: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Explanation {
    conflicts: Vec<Lock>,
    examples: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct Args {
    lock: Lock,
}

pub fn run(args: Args) -> Result<()> {
    let lock = args.lock;

    let content = include_str!("../resources/lock-explanations.yaml");
    let explanations: HashMap<Lock, Explanation> = serde_yaml::from_str(content)?;

    let template = include_str!("../resources/lock-explanation-template.tera.md");

    let mut tera = Tera::default();
    tera.add_raw_template(TEMPLATE_NAME, template)?;

    let explanation = explanations
        .get(&lock)
        .ok_or_else(|| eyre!("failed to get explanation for {lock}"))?;

    let context = TemplateContext {
        lock,
        conflicts: explanation.conflicts.clone(),
        examples: explanation.examples.clone(),
        blocked_examples: explanation
            .conflicts
            .iter()
            .flat_map(|conflict| explanations.get(conflict).unwrap().examples.iter().cloned())
            .collect(),
    };

    let context = Context::from_serialize(context)?;
    tera.render_to(TEMPLATE_NAME, &context, std::io::stdout())?;

    Ok(())
}
