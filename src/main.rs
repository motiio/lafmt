use lafmt::parser;

use std::{fs::File, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./tests/001.sql").expect("File not fount");
    let mut f = file;
    let mut query: String = String::new();
    f.read_to_string(&mut query)?;
    let mut p = parser::Parser::new();
    let kek = p.parse(&query);
    println!("{:?}", kek);

    println!("{}", &query);

    Ok(())
}
