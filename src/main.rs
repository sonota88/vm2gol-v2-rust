use std::env;

mod types;
mod utils;
mod json;
mod json_tester;
mod lexer;
mod parser;
mod codegen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd: &str = args[1].as_str();

    match cmd {
        "test-json" => json_tester::test_json(),
        "lex" => lexer::tokenize(),
        "parse" => parser::parse(),
        "codegen" => codegen::main(),
        _ => println!("invalid command ({})", cmd),
    }
}
