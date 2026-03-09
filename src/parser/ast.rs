// The building blocks of YaraT logic

#[derive(Debug, Clone)]
pub enum Statement {
    // Represents: sender_balance = 5000.00 USD
    Assignment {
        identifier: String,
        value: Expression,
    },
    
    // Represents: asset USD = Fiat(precision: 2)
    AssetDeclaration {
        ticker: String,
        precision: f64,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    // Represents a pure financial value: 5000.00 USD
    MoneyLiteral {
        amount: f64,
        currency: String,
    },
    
    // Represents a reference to another variable
    Identifier(String),
}

// A full YaraT program is just a list of Statements
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}