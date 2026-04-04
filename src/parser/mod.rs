pub mod ast;
use crate::lexer::Lexer;
use crate::lexer::token::Token;
use ast::{Program, Statement, Expression, Operator, BlockStatement};
use rust_decimal::Decimal;

#[derive(PartialEq, PartialOrd)]
enum Precedence { Lowest = 1, Equals = 2, LessGreater = 3, Sum = 4, Product = 5 }

pub struct Parser {
    lexer: Lexer, current_token: Token, peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser { lexer, current_token, peek_token }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };
        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() { 
                program.statements.push(stmt); 
            } else {
                // THE FIX: Stop silently dropping lines! Print exactly where the syntax broke.
                println!("⚠️ PARSER WARNING: Ignored invalid syntax starting at '{:?}'", self.current_token);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current_token {
            Token::AssetKeyword => self.parse_asset_declaration(),
            Token::If => self.parse_if_statement(),
            Token::Transaction => self.parse_transaction_declaration(),
            Token::Identifier(_) => {
                if self.peek_token == Token::OpenParen { self.parse_transaction_call() } 
                else { self.parse_assignment() }
            }
            _ => None,
        }
    }

    fn parse_assignment(&mut self) -> Option<Statement> {
        let identifier = match &self.current_token { Token::Identifier(id) => id.clone(), _ => return None };
        self.next_token(); if self.current_token != Token::Assign { return None; }
        self.next_token(); 
        Some(Statement::Assignment { identifier, value: self.parse_expression(Precedence::Lowest)? })
    }

    fn parse_asset_declaration(&mut self) -> Option<Statement> {
        self.next_token(); let ticker = match &self.current_token { Token::CurrencyTicker(t) => t.clone(), _ => return None };
        self.next_token(); if self.current_token != Token::Assign { return None; }
        self.next_token(); if let Token::Identifier(name) = &self.current_token { if name != "Fiat" { return None; } } else { return None; }
        self.next_token(); if self.current_token != Token::OpenParen { return None; }
        self.next_token(); if let Token::Identifier(name) = &self.current_token { if name != "precision" { return None; } } else { return None; }
        self.next_token(); if self.current_token != Token::Colon { return None; }
        self.next_token(); let precision = match self.current_token { Token::MoneyLiteral(val) => val.to_string().parse::<u32>().unwrap_or(2), _ => return None };
        self.next_token(); if self.current_token != Token::CloseParen { return None; }
        Some(Statement::AssetDeclaration { ticker, precision })
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        self.next_token(); let condition = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token != Token::LBrace { return None; }
        self.next_token(); let consequence = self.parse_block_statement();
        let mut alternative = None;
        if self.peek_token == Token::Else {
            self.next_token(); if self.peek_token != Token::LBrace { return None; }
            self.next_token(); alternative = Some(self.parse_block_statement());
        }
        Some(Statement::IfStatement { condition, consequence, alternative })
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = Vec::new();
        self.next_token();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() { statements.push(stmt); }
            self.next_token();
        }
        BlockStatement { statements }
    }

    fn parse_transaction_declaration(&mut self) -> Option<Statement> {
        self.next_token(); 
        let name = match &self.current_token { Token::Identifier(n) => n.clone(), _ => return None };
        self.next_token(); if self.current_token != Token::OpenParen { return None; }
        self.next_token();
        
        let mut parameters = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::EOF {
            let p_name = match &self.current_token { Token::Identifier(n) => n.clone(), _ => return None };
            self.next_token();
            if self.current_token != Token::Colon { return None; } 
            self.next_token();
            let p_type = match &self.current_token { 
                Token::CurrencyTicker(t) => t.clone(), 
                Token::Identifier(t) => t.clone(), 
                _ => return None 
            };
            parameters.push((p_name, p_type));
            self.next_token();
            if self.current_token == Token::Comma { self.next_token(); }
        }
        
        self.next_token(); if self.current_token != Token::LBrace { return None; }
        let body = self.parse_block_statement();
        Some(Statement::TransactionDeclaration { name, parameters, body })
    }

    fn parse_transaction_call(&mut self) -> Option<Statement> {
        let name = match &self.current_token { Token::Identifier(n) => n.clone(), _ => return None };
        self.next_token(); self.next_token(); 
        let mut arguments = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::EOF {
            if let Some(expr) = self.parse_expression(Precedence::Lowest) { arguments.push(expr); }
            self.next_token();
            if self.current_token == Token::Comma { self.next_token(); }
        }
        Some(Statement::TransactionCall { name, arguments })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = self.parse_primary()?;
        while self.peek_token != Token::EOF && precedence < self.peek_precedence() {
            self.next_token();
            let operator = match self.current_token {
                Token::Plus => Operator::Plus, Token::Minus => Operator::Minus,
                Token::Asterisk => Operator::Multiply, Token::Slash => Operator::Divide,
                Token::LessThan => Operator::LessThan, Token::GreaterThan => Operator::GreaterThan,
                Token::Equal => Operator::Equal, Token::NotEqual => Operator::NotEqual,
                _ => return Some(left),
            };
            let current_precedence = self.peek_precedence();
            self.next_token();
            let right = self.parse_expression(current_precedence)?;
            left = Expression::BinaryOperation { left: Box::new(left), operator, right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::MoneyLiteral(amount) => {
                let val = *amount;
                self.next_token(); 
                // THE UPGRADE: Parse the @ symbol for strict currency typing
                if self.current_token == Token::At {
                    self.next_token();
                    if let Token::CurrencyTicker(currency) = &self.current_token {
                        return Some(Expression::MoneyLiteral { amount: val, currency: currency.clone() });
                    } else {
                        return None;
                    }
                }
                // Fallback: legacy syntax without @
                if let Token::CurrencyTicker(currency) = &self.current_token {
                    Some(Expression::MoneyLiteral { amount: val, currency: currency.clone() })
                } else { None }
            }
            Token::Identifier(id) => Some(Expression::Identifier(id.clone())),
            Token::True => Some(Expression::BooleanLiteral(true)),
            Token::False => Some(Expression::BooleanLiteral(false)),
            _ => None,
        }
    }
}