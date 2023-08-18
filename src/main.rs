#![allow(dead_code, unused_imports)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::str::FromStr;

mod commands;
mod dirtree;
mod errors;

use dirtree::DirTree;

use crate::dirtree::DirNode;
use crate::errors::ApplicationError;

fn main() -> Result<(), ApplicationError> {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].to_string();

    let lines = match read_file(filename) {
        Ok(lines) => lines,
        Err(e) => return Err(e),
    };

    let mut tree = DirTree::new();

    for line in lines {
        if let Ok(instruction) = line {
            let _result = tree.execute(instruction);
        }
    }

    Ok(())
}

fn read_file(filename: String) -> Result<Lines<BufReader<File>>, ApplicationError> {
    let file = File::open(filename);
    match file {
        Ok(ref _file) => Ok(BufReader::new(file.unwrap()).lines()),
        Err(_e) => return Err(ApplicationError::MissingFileError),
    }
}
