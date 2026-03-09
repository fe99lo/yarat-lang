#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Finance Primitives
    MoneyLiteral(f64),       
    CurrencyTicker(String),  
    
    // Text & Variables
    Identifier(String),      
    
    // Operators
    Assign,                  // =
    
    // System
    EOF,                     // End of File
    Illegal(char),           // Catches invalid characters securely
}