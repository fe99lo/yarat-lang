#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // -------------------------
    // EXISTING FOUNDATION
    // -------------------------
    // Finance Primitives
    MoneyLiteral(f64),       
    CurrencyTicker(String),
    AssetKeyword,            
    
    // Text & Variables
    Identifier(String),      
    
    // Math Operators
    Plus,                    
    Minus,                   
    
    // System Operators & Punctuation
    Assign,                  
    OpenParen,               
    CloseParen,              
    Colon,                   
    
    // -------------------------
    // NEW V1.0 ADDITIONS
    // -------------------------
    // Booleans
    True,
    False,

    // Advanced Math
    Asterisk, // * (For multipliers, like calculating 2% fees)
    Slash,    // / (For division)
    
    // Comparisons
    LessThan,    // <
    GreaterThan, // >
    Equal,       // ==
    NotEqual,    // !=
    
    // Block Punctuation
    LBrace,      // { (Starts an if-block)
    RBrace,      // } (Ends an if-block)
    Comma,       // , (Separates items)
    
    // Control Flow Keywords
    If,
    Else,
    Transaction, // For future Phase 6 grouping
    // -------------------------

    // Control
    EOF,                     
    Illegal(char),           
}