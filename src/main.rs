mod helpers;
mod node;

use helpers::read_file;
use node::Node;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a filename argument.")
    }
    let lines = read_file(&args[1])?;

    let mut root = Node::new();

    for line in lines.flatten() {
        let line = line.trim();
        if !line.is_empty() {
            println!("{line}");
            if let Err(e) = root.execute(line) {
                println!("{e:?}");
            }
        }
    }

    Ok(())
}

pub enum Error {
    IO(std::io::Error),
    Missing {
        op: &'static str,
        path: String,
        missing: String,
    },
    Exists {
        op: &'static str,
        path: String,
    },
    ParamCount(usize, String),
    UnknownCommand(String),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO: {e}"),
            Self::Missing { op, path, missing } => {
                write!(f, "Cannot {op} {path} - {missing} does not exist")
            }
            Self::Exists { op, path } => write!(f, "Cannot {op} {path} - already exists"),
            Self::ParamCount(expected, cmd) => {
                write!(f, "Expected {expected} parameters for command: {cmd:?}")
            }
            Self::UnknownCommand(cmd) => {
                write!(f, "Unknown command: {cmd:?}")
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
