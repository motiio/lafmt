use lafmt::parser::Parser;
use lafmt::string_buf::StringBuf;
use std::{fs::File, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("./tests/001.sql").expect("File not fount");
    let mut buff: String = String::new();
    f.read_to_string(&mut buff)?;
    let s_buf = StringBuf::new(&buff);
    let mut s_buf_iter = s_buf.iter_from(9);
    if let Some(token) = s_buf_iter.fetch_to_delim(" ") {
        println!("{}", token);
    }

    for ch in s_buf.iter() {
        println!("{}", ch);
    }
    // let mut parser = Parser::new(&buff);

    // let _ = parser.parse();
    // println!("{:?}", parser.tokens());

    Ok(())
}
