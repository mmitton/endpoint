use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use super::ApplicationError;

pub fn read_file(filename: String) -> Result<Lines<BufReader<File>>, ApplicationError> {
    let file = File::open(filename);
    match file {
        Ok(ref _file) => Ok(BufReader::new(file.unwrap()).lines()),
        Err(_e) => return Err(ApplicationError::new("Could not open file ${filename}")),
    }
}
