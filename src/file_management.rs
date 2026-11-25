use std::fs;

pub fn get_contents(path: &str) -> Result<String, std::io::Error> {

    let contents = fs::read_to_string(path)?;

    Ok(contents) // no semicolon or rust molests me

}