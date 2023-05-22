use std::io;
use std::io::Write;
use std::process::exit;
use std::result::Result as StdResult;

use yansi::{Color, Paint};

#[derive(Debug)]
pub enum Error {
    ChildProcess(String),
    Msg(String),
}

pub type Result<T> = StdResult<T, Error>;

fn error(msg: &str) {
    writeln!(
        io::stderr(),
        "{} {msg}",
        Paint::new("error:").fg(Color::Red).bold()
    )
    .unwrap_or_default();
}

impl Error {
    /// Print the error message to stderr and exit.
    pub fn exit(self) -> ! {
        exit(match self {
            Error::Msg(desc) => {
                error(&desc);
                1
            }
            Error::ChildProcess(desc) => {
                error(&format!("child process error: {desc}"));
                2
            }
        });
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Msg(e.to_string())
    }
}
