use std::env;
use std::fs;
use std::process;

mod lexer;
use lexer::Lexer;
use lexer::token::Token;

fn main() {
    // 1. Capture the commands typed into the terminal
    let args: Vec<String> = env::args().collect();

    // 2. Enforce strict CLI usage (e.g., `yarat run file.yt`)
    if args.len() < 3 || args[1] != "run" {
        eprintln!("❌ Error: Invalid command.");
        eprintln!("Usage: yarat run <filepath.yt>");
        process::exit(1); // Exit securely without crashing
    }

    let file_path = &args[2];

    println!("========================================");
    println!("  YaraT Compiler Engine v0.1.0 Started  ");
    println!("========================================");
    println!("Loading file: {}\n", file_path);

    // 3. Read the actual file from the hard drive
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("❌ Fatal Error: Could not read file '{}'", file_path);
        eprintln!("System Message: {}", err);
        process::exit(1);
    });

    // 4. Pass the file's contents into our Lexer
    let mut lexer = Lexer::new(&source_code);
    
    // 5. Scan the tokens
    loop {
        let token = lexer.next_token();
        println!("Found Token: {:?}", token);
        
        if token == Token::EOF {
            break;
        }
    }
}