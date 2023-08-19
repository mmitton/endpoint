use std::env;
use std::fmt;

mod helpers;
use helpers::read_file;

mod dirtree;
use dirtree::DirTree;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a filename argument.")
    }
    let filename = args[1].to_string();

    let lines = match read_file(filename) {
        Ok(lines) => lines,
        Err(e) => panic!("{e}"),
    };

    let mut tree = DirTree::new();

    for line in lines {
        if let Ok(instruction) = line {
            println!("{instruction}");
            match tree.execute(instruction) {
                Ok(_) => (),
                Err(e) => println!("{e}"),
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ApplicationError {
    error: String,
}
impl ApplicationError {
    pub fn new(error_string: &str) -> ApplicationError {
        ApplicationError {
            error: error_string.to_string(),
        }
    }
}
impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
