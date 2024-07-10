use regex::Regex;
use std::{fs::File, io::Read};
use lafmt::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("./tests/001.sql").expect("File not fount");
    let mut buff: String = String::new();
    f.read_to_string(&mut buff)?;
    let mut parser = Parser::new(&buff);
    let _ = parser.parse();
    println!("{:?}", parser.tokens());

    Ok(())
}
