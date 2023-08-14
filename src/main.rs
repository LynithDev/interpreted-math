use std::{fs, process::exit};

use lynith_lang::{
    parsers::{infix_parser::InfixParser, Parser, syntax_parser::SyntaxParser}, 
    evaluators::{postfix_evaluator::PostfixEvaluator, Evaluator}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Missing input file");
        exit(1);
    }

    let input_file_path = &args[1];

    let content = fs::read_to_string(input_file_path)?;
    
    let parser = SyntaxParser::parse(&content)?;
    println!("{:?}", parser);

    // let parsed_content = InfixParser::parse(&content)?;
    
    // println!("Postfix: {}", parsed_content);
    // let evaluated_content = PostfixEvaluator::eval(parsed_content)?;

    // println!("{}", evaluated_content);

    Ok(())
}
