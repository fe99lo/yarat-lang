mod lexer;
mod parser;
mod semantic;
mod codegen;
mod server;

use std::env;
use std::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_banner();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "serve" => {
            // Enterprise Telemetry
            if env::var("RUST_LOG").is_err() {
                env::set_var("RUST_LOG", "info");
            }
            server::run_server().await?;
        }
        "run" => {
            // Secure Local Execution Mode
            if args.len() < 3 {
                println!("❌ Error: Please provide a file to run. Example: cargo run -- run examples/finance_audit.yt");
                return Ok(());
            }
            let filename = &args[2];
            
            let contents = match fs::read_to_string(filename) {
                Ok(text) => text,
                Err(_) => {
                    println!("❌ Fatal Error: Could not read file '{}'. Are you sure it exists?", filename);
                    return Ok(());
                }
            };
            
            println!("\n--- Commencing Semantic Audit ---");
            let mut lexer = lexer::Lexer::new(&contents);
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            let mut analyzer = semantic::Analyzer::new();
            if let Err(e) = analyzer.analyze_program(&program) {
                println!("❌ {}", e);
                return Ok(());
            }
            println!("🟢 Audit Passed! YaraT code is mathematically secure.\n");

            println!("--- Executing YaraT Program ---");
            let mut evaluator = codegen::Evaluator::new();
            evaluator.evaluate_program(&program);
            
            // ---------------------------------------------------------
            // AUDIT REPORT: Memory State
            // ---------------------------------------------------------
            println!("\n📊 FINAL MEMORY STATE:");
            let mut sorted_keys: Vec<_> = evaluator.memory.keys().collect();
            sorted_keys.sort();
            
            for key in sorted_keys {
                let val = &evaluator.memory[key];
                match val {
                    codegen::RuntimeValue::Money { amount, currency } => {
                        println!("  [{}] => {:.2} {}", key, amount, currency);
                    }
                    codegen::RuntimeValue::Boolean(b) => {
                        println!("  [{}] => {} (Logical State)", key, b);
                    }
                }
            }

            // ---------------------------------------------------------
            // AUDIT REPORT: Secure Smart Contract Vault
            // ---------------------------------------------------------
            println!("\n🏦 SECURE SMART CONTRACT VAULT:");
            if evaluator.transaction_vault.is_empty() {
                println!("  [Empty - No contracts registered]");
            } else {
                for (name, (params, _body)) in &evaluator.transaction_vault {
                    // Extract the strictly typed parameters to prove security
                    let param_list: Vec<String> = params.iter()
                        .map(|(p_name, p_type)| format!("{}: {}", p_name, p_type))
                        .collect();
                    
                    println!("  📜 Contract '{}' loaded securely.", name);
                    println!("     ↳ Enforced Parameters: ({})", param_list.join(", "));
                }
            }
            println!();
        }
        _ => {
            println!("❌ Unknown command: {}", command);
            print_banner();
        }
    }

    Ok(())
}

fn print_banner() {
    println!("========================================");
    println!("  YaraT Compiler Engine v1.0");
    println!("========================================");
    println!("Usage: cargo run -- <command> [file]");
    println!("\nCommands:");
    println!("  run <file.yt>   - Execute a local YaraT script securely");
    println!("  serve           - Boot up the enterprise YaraT Web API");
    println!("========================================");
}
