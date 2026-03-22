use crate::config::Config;
use crate::store::Store;
use anyhow::{Context, Result, anyhow};
use std::fs;

pub fn run(id: Option<&str>, content: String) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let issue = match id {
        Some(id) => store.find(id)?,
        None => store
            .current()
            .context("Failed to get current issue")?
            .ok_or_else(|| anyhow!("No current issue"))?,
    };

    fs::write(&issue.path, &content)
        .with_context(|| format!("Failed to write issue: {}", issue.path.display()))?;

    println!("Updated {}: {}", issue.id, issue.title());

    Ok(())
}
