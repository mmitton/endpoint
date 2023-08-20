use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

use super::Error;

pub fn read_file(filename: impl AsRef<Path>) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
