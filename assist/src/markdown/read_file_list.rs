use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};

pub fn read_file_list(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut file_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        file_list.push(line);
    }

    Ok(file_list)
}

