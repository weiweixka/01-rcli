use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let readr: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(readr)
}
