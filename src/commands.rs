use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

use yansi::{Color, Paint};

use crate::args::Platform;
use crate::consts::{ALIAS_PAGES, MORE_INFORMATION};
use crate::error::{Error, Result};
use crate::repo::Repository;
use crate::util::{
    browser, cmd, edit_page, existing_page_path, get_output, infoln, is_success, page_path, spawn,
    warnln,
};

pub fn update() -> Result<()> {
    let branch = get_output(&mut cmd!("git", "branch", "--show-current"))?;

    if branch != "main" {
        spawn(&mut cmd!("git", "checkout", "main"))?;
    }

    spawn(&mut cmd!("git", "pull", "upstream", "main"))?;

    if is_success(&mut cmd!(
        "git",
        "merge-base",
        "--is-ancestor",
        "HEAD",
        "@{u}"
    ))? {
        infoln!("your fork is up to date, not pushing");
    } else {
        spawn(&mut cmd!("git", "push"))?;
    }

    if branch != "main" {
        spawn(&mut cmd!("git", "checkout", branch))?;
    }

    Ok(())
}

pub fn view(repo_path: &Path, page: &[String], language: &str, platform: &Platform) -> Result<()> {
    let page = page.join("-").to_lowercase();
    let path = existing_page_path(repo_path, platform, language, &page)?;

    spawn(&mut cmd!("tldr", "--render", path))
}

pub fn destroy(repo_path: &Path) -> Result<()> {
    warnln!(
        "you {} be able to recover changes that have not been pushed to GitHub",
        Paint::new("WILL NOT").fg(Color::Red).bold()
    );
    print!("Press Enter to confirm. ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut String::new())?;

    infoln!("removing '{}'...", repo_path.display());
    fs::remove_dir_all(repo_path)?;

    let config_dir = Repository::locate_config();
    infoln!("removing '{}'...", config_dir.parent().unwrap().display());
    fs::remove_dir_all(config_dir.parent().unwrap())?;

    Ok(())
}

pub fn add(
    repo_path: &Path,
    page: &str,
    doc_url: &str,
    language: &str,
    platform: &Platform,
) -> Result<()> {
    let page = page.to_lowercase();
    let page_path = page_path(repo_path, platform, language, &page);

    if page_path.is_file() {
        return Err(Error::Msg(format!(
            "page '{language}/{platform}/{page}' already exists"
        )));
    }

    infoln!("creating page '{language}/{platform}/{page}'...");
    write!(
        File::create(&page_path)?,
        "# {page}\n\n\
            >\n>\n> {}: <{doc_url}>.\n\n-\n\n``\n",
        MORE_INFORMATION[language]
    )?;

    edit_page(&page_path)
}

pub fn alias(
    repo_path: &Path,
    new_page: &str,
    alias_of: &str,
    language: &str,
    platform: &Platform,
) -> Result<()> {
    let new_page = new_page.to_lowercase();
    let alias_of = alias_of.to_lowercase();

    if !ALIAS_PAGES.contains_key(language) {
        return Err(Error::Msg(format!(
            "the alias page has not been translated to '{language}' yet"
        )));
    }

    let new_page_path = page_path(repo_path, platform, language, &new_page);
    if new_page_path.is_file() {
        return Err(Error::Msg(format!(
            "page '{language}/{platform}/{new_page}' already exists"
        )));
    }

    let alias: String = ALIAS_PAGES[language].replace("example", &alias_of);

    infoln!("creating alias ({language}/{platform}): '{new_page}' => '{alias_of}'");
    fs::create_dir_all(new_page_path.parent().unwrap())?;
    write!(File::create(&new_page_path)?, "{alias}")?;

    edit_page(&new_page_path)
}

pub fn edit(repo_path: &Path, page: &[String], language: &str, platform: &Platform) -> Result<()> {
    edit_page(&existing_page_path(
        repo_path,
        platform,
        language,
        &page.join("-").to_lowercase(),
    )?)
}

pub fn branch(branch: &str) -> Result<()> {
    if is_success(&mut cmd!(
        "git",
        "show-ref",
        "--verify",
        format!("refs/heads/{branch}")
    ))? {
        infoln!("branch '{branch}' exists, checking it out...");
        spawn(&mut cmd!("git", "checkout", branch))
    } else {
        infoln!("branch '{branch}' does not exist, creating it...");
        spawn(&mut cmd!("git", "checkout", "-b", branch))
    }
}

pub fn translate(repo_path: &Path, page: &str, language: &str, platform: &Platform) -> Result<()> {
    let page = page.to_lowercase();

    if language == "en" {
        warnln!("no language specified, running 'edit' instead");
        edit_page(&existing_page_path(repo_path, platform, language, &page)?)?;
        return Ok(());
    }

    let page_path = page_path(repo_path, platform, language, &page);

    if page_path.is_file() {
        infoln!("this translation already exists, running 'edit' instead");
        edit_page(&page_path)?;
        return Ok(());
    }

    let en_page_path = existing_page_path(repo_path, platform, "en", &page).map_err(|_| {
        Error::Msg(format!(
            "page 'en/{platform}/{page}' does not exist, create it first"
        ))
    })?;

    let page_str =
        fs::read_to_string(en_page_path)?.replace("More information", MORE_INFORMATION[language]);

    infoln!("creating page '{language}/{platform}/{page}'...");
    fs::create_dir_all(page_path.parent().unwrap())?;
    write!(File::create(&page_path)?, "{page_str}")?;

    edit_page(&page_path)
}

pub fn pull_request(fork: &str) -> Result<()> {
    let branch = get_output(&mut cmd!("git", "branch", "--show-current"))?;

    if branch == "main" {
        return Err(Error::Msg(
            "the 'main' branch is checked out. Switch branches using 'tlt branch' first."
                .to_string(),
        ));
    }

    if is_success(&mut cmd!(
        "git",
        "merge-base",
        "--is-ancestor",
        "HEAD",
        "@{u}"
    ))? {
        infoln!("origin is up to date, not pushing");
    } else {
        spawn(&mut cmd!("git", "push", "-u", "origin", &branch))?;
    }

    browser(&format!(
        "https://github.com/tldr-pages/tldr/compare/main...{}:{branch}",
        fork.replace('/', ":")
    ))?;

    Ok(())
}
