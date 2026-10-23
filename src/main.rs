use std::{
    env, fs::File, io::{BufReader, Read}, str::Chars
};

mod lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let file_path = args.nth(1).ok_or("Missing input file path")?;
    let mut reader = BufReader::new(File::open(file_path)?);

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let mut lexer = lexer::Lexer::new(&content);

    while let Some(t) = lexer.next() {
        println!("{t:?}");
    }

    Ok(())
}
