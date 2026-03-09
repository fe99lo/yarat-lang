#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Finance Primitives
    MoneyLiteral(f64),       
    CurrencyTicker(String),
    AssetKeyword,            // <-- Added this  
    
    // Text & Variables
    Identifier(String),      
    
    // Operators & Punctuation
    Assign,                  // =
    OpenParen,               // <-- Added this (
    CloseParen,              // <-- Added this )
    Colon,                   // <-- Added this :
    
    // System
    EOF,                     
    Illegal(char),           
}