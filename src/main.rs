use std::{
    env, fs::File, io::{BufReader, Read}
};

mod lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let file_path = args.nth(1).ok_or("Missing input file path")?;
    let mut reader = BufReader::new(File::open(&file_path)?);

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let mut lexer = lexer::Lexer::new(&content, file_path);

    while let Some(t) = lexer.next() {
        match t {
            Ok(ref t) => {
                let lexer::Location { file_path, line, column } = &t.loc;
                println!("{file_path}:{}:{}: Token: {:?}", line + 1, column + 1, t.kind);
            },
            Err(ref e) => {
                let (name, loc) = match &e {
                    lexer::TokenError::NumberError(loc) => ("invalid number", loc),
                    lexer::TokenError::StringError(loc) => ("unterminated string", loc),
                    lexer::TokenError::UnknownTokenError(loc) => ("unknown token", loc),
                    lexer::TokenError::CommentError(loc) => ("unterminated multiline comment", loc),
                };
                let lexer::Location { file_path, line, column } = loc;
                eprintln!("{file_path}:{}:{}: ERROR: {name}", line + 1, column + 1);
            }
        }
    }

    Ok(())
}
