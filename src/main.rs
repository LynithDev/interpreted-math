use std::{fs, process::exit};

use lynith_lang::parsers::{Parser, syntax_parser::SyntaxParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Missing input file");
        exit(1);
    }

    let input_file_path = &args[1];

    let content = fs::read_to_string(input_file_path)?;
    
    let value = SyntaxParser::parse(&content)?;
    match value {
        Some(value) => println!("{}", value),
        None => exit(1)
    }

    Ok(())
}
