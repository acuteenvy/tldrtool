#![warn(unused)]
#![warn(clippy::all, clippy::pedantic, clippy::style)]

mod args;
mod commands;
mod consts;
mod error;
mod repo;
mod util;

use std::{env, io};

use clap::Parser;
use is_terminal::IsTerminal;

use crate::args::{Cli, Commands};
use crate::consts::MORE_INFORMATION;
use crate::error::{Error, Result};
use crate::repo::Repository;
use crate::util::{cmd, spawn};

fn run() -> Result<()> {
    let cli = Cli::parse();

    #[cfg(target_os = "windows")]
    let color_support = yansi::Paint::enable_windows_ascii();
    #[cfg(not(target_os = "windows"))]
    let color_support = true;

    if !color_support || env::var("NO_COLOR").is_ok() || !io::stdout().is_terminal() {
        yansi::Paint::disable();
    }

    let repo = Repository::get()?;
    if !repo.ensure_exists()? {
        // Quit if the repository did not exist.
        return Ok(());
    }

    env::set_current_dir(&repo.path)?;

    let platform = cli.platform.unwrap_or_default();
    let language = cli.language.unwrap_or("en".to_string());

    if !MORE_INFORMATION.contains_key(language.as_str()) {
        return Err(Error::Msg(format!(
            "there are no translations in '{language}'"
        )));
    }

    match cli.command {
        Commands::Run(args) => spawn(cmd!(&args[0]).args(&args[1..]))?,
        Commands::Destroy => commands::destroy(&repo.path)?,
        Commands::Update => commands::update()?,
        Commands::View { page } => commands::view(&repo.path, &page, &language, &platform)?,
        Commands::Edit { page } => commands::edit(&repo.path, &page, &language, &platform)?,
        Commands::Branch { branch } => commands::branch(branch)?,
        Commands::PullRequest => commands::pull_request(&repo.fork)?,
        Commands::Add { page, doc_url } => {
            commands::add(&repo.path, &page, &doc_url, &language, &platform)?;
        }
        Commands::Alias { new_page, alias_of } => {
            commands::alias(&repo.path, &new_page, &alias_of, &language, &platform)?;
        }
        Commands::Translate { page } => {
            commands::translate(&repo.path, &page, &language, &platform)?;
        }
        Commands::Setup => {
            Repository::setup_config()?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        e.exit();
    }
}
