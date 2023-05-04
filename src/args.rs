use std::fmt::Display;
use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Default, Clone)]
pub enum Platform {
    Linux,
    OsX,
    Windows,
    Android,
    SunOs,
    #[default]
    Common,
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linux" => Ok(Self::Linux),
            "macos" | "osx" => Ok(Self::OsX),
            "windows" => Ok(Self::Windows),
            "android" => Ok(Self::Android),
            "sunos" => Ok(Self::SunOs),
            "common" => Ok(Self::Common),
            _ => Err(format!("invalid platform '{s}' (possible values: linux, macos, osx, windows, android, sunos, common)'"))
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Linux => "linux",
                Self::OsX => "osx",
                Self::Windows => "windows",
                Self::Android => "android",
                Self::SunOs => "sunos",
                Self::Common => "common",
            }
        )
    }
}

#[derive(Parser)]
#[command(
    arg_required_else_help = true,
    about,
    version,
    after_help = "See 'man tlt' or https://acuteenvy.github.io/tldrtool for more information.",
    help_template = "{before-help}{name} {version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}",
    override_usage = "\x1b[1mtlt\x1b[0m [OPTIONS] <COMMAND>...\n       \
\x1b[1mtlt\x1b[0m <SHELL_COMMAND> [ARGS]..."
)]
pub struct Cli {
    /// Specify the platform to use [linux, macos/osx, windows, android, sunos, common].
    #[arg(short, long)]
    pub platform: Option<Platform>,

    /// Specify the language to use.
    #[arg(short = 'L', long, value_name = "LANGUAGE")]
    pub language: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Run a command inside the tldr repository.
    #[command(external_subcommand)]
    Run(Vec<String>),

    /// Create a new page.
    Add { page: String, doc_url: String },

    /// Create an alias page.
    #[command(visible_alias = "al")]
    Alias { new_page: String, alias_of: String },

    /// Translate a page.
    #[command(visible_alias = "t")]
    Translate { page: String },

    /// Edit a page.
    #[command(visible_alias = "e")]
    Edit {
        #[arg(required = true)]
        page: Vec<String>,
    },

    /// Update your fork's main branch from upstream.
    #[command(visible_alias = "u")]
    Update,

    /// Show, switch or create git branches.
    #[command(visible_alias = "b")]
    Branch { branch: Option<String> },

    /// Run 'tldr --render' on a page.
    #[command(visible_alias = "v")]
    View {
        #[arg(required = true)]
        page: Vec<String>,
    },

    /// Create a pull request to tldr-pages from the current branch.
    #[command(visible_alias = "pr")]
    PullRequest,

    /// Recreate the config file.
    Setup,

    /// Remove the repository and the config file.
    Destroy,
}
