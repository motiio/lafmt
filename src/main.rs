use lafmt::tokenizer::tokenize;
use std::{fs::File, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("./tests/001.sql").expect("File not fount");
    let mut query: String = String::new();
    f.read_to_string(&mut query)?;
    println!("{}", &query);
    let tokens = tokenize(&query)?;
    println!("{:?}", tokens);
    // let mut parser = Parser::new(&buff);

    // let _ = parser.parse(); println!("{:?}", parser.tokens());

    Ok(())
}
