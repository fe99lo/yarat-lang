pub mod ast;
use crate::lexer::Lexer;
use crate::lexer::token::Token;
use ast::{Program, Statement, Expression};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    // 1. Boot up the Parser and load the first two tokens
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    // 2. Step forward through the code
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // 3. The Master Loop: Read the whole file and build the AST
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };

        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    // 4. Determine what kind of logic we are looking at
    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current_token {
            Token::AssetKeyword => self.parse_asset_declaration(),
            Token::Identifier(_) => self.parse_assignment(),
            _ => None, // If we hit unknown syntax, safely ignore for now
        }
    }

    // 5. Build an Asset Declaration: `asset USD = Fiat(precision: 2)`
    fn parse_asset_declaration(&mut self) -> Option<Statement> {
        self.next_token(); // Move past 'asset' to the ticker

        let ticker = match &self.current_token {
            Token::CurrencyTicker(t) => t.clone(),
            _ => return None,
        };

        // Fast-forward to find the exact decimal precision required
        while self.current_token != Token::EOF {
            if let Token::MoneyLiteral(val) = self.current_token {
                return Some(Statement::AssetDeclaration {
                    ticker,
                    precision: val,
                });
            }
            self.next_token();
        }
        None
    }

    // 6. Build a Financial Assignment: `sender_balance = 5000.00 USD`
    fn parse_assignment(&mut self) -> Option<Statement> {
        let identifier = match &self.current_token {
            Token::Identifier(id) => id.clone(),
            _ => return None,
        };

        self.next_token(); // Move to '='
        if self.current_token != Token::Assign { return None; }

        self.next_token(); // Move to the financial value
        let amount = match self.current_token {
            Token::MoneyLiteral(val) => val,
            _ => return None,
        };

        self.next_token(); // Move to the currency ticker
        let currency = match &self.current_token {
            Token::CurrencyTicker(t) => t.clone(),
            _ => return None,
        };

        // Securely bind the amount and the currency together in memory
        Some(Statement::Assignment {
            identifier,
            value: Expression::MoneyLiteral { amount, currency },
        })
    }
}