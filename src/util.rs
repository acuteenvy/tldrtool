use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;

use yansi::{Color, Paint};

use crate::args::Platform;
use crate::error::{Error, Result};

/// Constructs  a `Command` from arguments.
macro_rules! cmd {
    ( $cmd:expr ) => {
        {
            let cmd = std::process::Command::new($cmd);
            cmd
        }
    };
    ( $cmd:expr, $( $arg:expr ),* ) => {
        {
            let mut cmd = std::process::Command::new($cmd);
            $( cmd.arg($arg); )*
            cmd
        }
    };
}

/// Prints a warning.
macro_rules! warnln {
    ( $( $arg:tt )*) => {
        {
            use std::io::Write;
            let mut lock = std::io::stderr().lock();
            write!(lock, "{} ", yansi::Paint::new("warning:").fg(yansi::Color::Yellow).bold())?;
            writeln!(lock, $($arg)*)?;
        }
    };
}

/// Prints a status message.
macro_rules! infoln {
    ( $( $arg:tt )*) => {
        {
            use std::io::Write;
            let mut lock = std::io::stderr().lock();
            write!(lock, "{} ", yansi::Paint::new("info:").fg(yansi::Color::Cyan).bold())?;
            writeln!(lock, $($arg)*)?;
        }
    };
}

pub(crate) use {cmd, infoln, warnln};

/// Spawns a child process and returns an error if it fails to spawn or exits with non-zero status.
pub fn spawn(cmd: &mut Command) -> Result<()> {
    writeln!(
        io::stderr(),
        "{} {} {}",
        Paint::new("running:").fg(Color::Blue).bold(),
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<Cow<str>>>()
            .join(" ")
    )?;

    let status = cmd
        .spawn()
        .map_err(|e| Error::ChildProcess(e.to_string()))?
        .wait()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::ChildProcess(status.to_string()))
    }
}

/// Spawns a child process with its output streams attached to `/dev/null`.
/// Returns `true` if the exit code is `0`, and `false` otherwise.
/// Returns an error if the process fails to spawn.
pub fn is_success(cmd: &mut Command) -> Result<bool> {
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    write!(
        io::stderr(),
        "{} {} {} ... ",
        Paint::new("checking status:").fg(Color::Magenta).bold(),
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<Cow<str>>>()
            .join(" ")
    )?;

    let status = cmd
        .spawn()
        .map_err(|e| Error::ChildProcess(e.to_string()))?
        .wait()?;
    if status.success() {
        writeln!(
            io::stderr(),
            "{}",
            Paint::new(status.code().unwrap()).fg(Color::Green).bold()
        )?;

        Ok(true)
    } else {
        writeln!(
            io::stderr(),
            "{}",
            Paint::new(status.code().unwrap()).fg(Color::Red).bold()
        )?;

        Ok(false)
    }
}

/// Spawns a child process and returns its stdout as a `String`, with the trailing newline stripped.
pub fn get_output(cmd: &mut Command) -> Result<String> {
    writeln!(
        io::stderr(),
        "{} {} {} ... ",
        Paint::new("capturing output:").fg(Color::Magenta).bold(),
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<Cow<str>>>()
            .join(" ")
    )?;

    let output = cmd.output()?;
    let status = output.status;

    if output.status.success() {
        let output_str = str::from_utf8(&output.stdout).unwrap();
        let output_str = output_str.strip_suffix('\n').unwrap_or(output_str);

        writeln!(
            io::stdout(),
            "{}",
            Paint::new(&output_str).fg(Color::Green).bold()
        )?;
        Ok(output_str.to_string())
    } else {
        writeln!(
            io::stderr(),
            "{}",
            Paint::new("ERROR").fg(Color::Red).bold()
        )?;
        Err(Error::ChildProcess(status.to_string()))
    }
}

/// Constructs a `PathBuf` to a page.
pub fn page_path(repo_path: &Path, platform: &Platform, language: &str, page: &str) -> PathBuf {
    let lang_dir = if language == "en" {
        "pages".to_string()
    } else {
        format!("pages.{language}")
    };

    PathBuf::from(repo_path)
        .join(lang_dir)
        .join(platform.to_string())
        .join(page.replace(' ', "-") + ".md")
}

/// Constructs a `PathBuf` to a page, and returns an error if it does not exist.
pub fn existing_page_path(
    repo_path: &Path,
    platform: &Platform,
    language: &str,
    page: &str,
) -> Result<PathBuf> {
    let path = page_path(repo_path, platform, language, page);

    if path.is_file() {
        Ok(path)
    } else {
        Err(Error::Msg(format!(
            "{language}/{platform}/{page}: page does not exist"
        )))
    }
}

/// Edits `path` using `$EDITOR` or `nano`.
pub fn edit_page(path: &Path) -> Result<()> {
    spawn(&mut cmd!(
        env::var("EDITOR").unwrap_or("nano".to_string()),
        path
    ))
}

/// Open `url` in the user's default browser.
pub fn browser(url: &str) -> Result<()> {
    #[cfg(target_os = "linux")]
    return spawn(&mut cmd!("xdg-open", url));

    #[cfg(target_os = "macos")]
    return spawn(&mut cmd!("open", url));

    #[cfg(target_os = "windows")]
    return spawn(&mut cmd!("explorer", url));
}
