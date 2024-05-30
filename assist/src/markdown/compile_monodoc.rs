use std::fs::File;
use std::io::{self, Read, Result};

pub fn compile_mono_doc(file_list: Vec<String>) -> Result<String> {
    let mut output = String::new();

    for file_name in file_list {
        let mut file = File::open(file_name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        output.push_str(&content);
        output.push_str("\n\n"); // Add a new line between files
    }

    Ok(output)
}

