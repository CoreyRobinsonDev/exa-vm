use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Register {
    Keyword(String),
    Number(i16)
}


#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Global,
    Local
}

#[derive(Debug)]
pub struct Exa {
    pub name: String,
    pub x: Register,
    pub t: Register,
    pub f: Register,
    pub m: Register,
    pointer: i16,
    mode: Mode,
    code: String

}

impl Exa {
    pub fn new(file: String) -> Self { 
        Self { 
            name: String::from("xa"), 
            x: Register::Number(0), 
            t: Register::Number(0), 
            f: Register::Number(0), 
            m: Register::Number(0), 
            pointer: 0, 
            mode: Mode::Global,
            code: read(file).unwrap_or(String::new())
        } 
    }

}

fn read(file: String) -> Result<String, std::io::Error> {
    let file = File::open(file)?;
    let mut buf_reader = BufReader::new(&file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    return Ok(contents);
}

