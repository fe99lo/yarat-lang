#![allow(dead_code)]

use rust_decimal::Decimal;

// ---------------------------------------------------------
// THE ROOT
// ---------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

// ---------------------------------------------------------
// THE STATEMENTS 
// ---------------------------------------------------------
#[derive(Debug, Clone)]
pub enum Statement {
    AssetDeclaration {
        ticker: String,
        precision: u32,
    },
    Assignment {
        identifier: String,
        value: Expression, 
    },
    IfStatement {
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>, 
    },
    // PHASE 6: Creating the strictly-typed reusable block
    TransactionDeclaration {
        name: String,
        // THE UPGRADE: Now stores BOTH the parameter name and its required currency/type
        parameters: Vec<(String, String)>, 
        body: BlockStatement,
    },
    // PHASE 6: Triggering the reusable block
    TransactionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus, Minus, Multiply, Divide,
    LessThan, GreaterThan, Equal, NotEqual,
}

#[derive(Debug, Clone)]
pub enum Expression {
    MoneyLiteral { amount: Decimal, currency: String },
    BooleanLiteral(bool), 
    Identifier(String),   
    BinaryOperation {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}