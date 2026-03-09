use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser; // <-- Bring the Architect online

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] != "run" {
        eprintln!("❌ Error: Invalid command.");
        eprintln!("Usage: yarat run <filepath.yt>");
        process::exit(1);
    }

    let file_path = &args[2];

    println!("========================================");
    println!("  YaraT Compiler Engine v0.1.0 Started  ");
    println!("========================================");
    println!("Loading file: {}\n", file_path);

    let source_code = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("❌ Fatal Error: Could not read file '{}'", file_path);
        eprintln!("System Message: {}", err);
        process::exit(1);
    });

    // 1. Boot up the Lexer (Phase 1)
    let lexer = Lexer::new(&source_code);
    
    // 2. Pass the Lexer into the Parser (Phase 2)
    let mut parser = Parser::new(lexer);

    // 3. Build the Abstract Syntax Tree
    let ast = parser.parse_program();

    // 4. Print the final mathematical structure to the terminal
    println!("{:#?}", ast);
}