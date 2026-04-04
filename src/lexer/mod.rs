pub mod token;
use token::Token;
use rust_decimal::Decimal;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() { '\0' } 
        else { self.input[self.read_position] }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => { if self.peek_char() == '=' { self.read_char(); Token::Equal } else { Token::Assign } }
            '!' => { if self.peek_char() == '=' { self.read_char(); Token::NotEqual } else { Token::Illegal(self.ch) } }
            '+' => Token::Plus, '-' => Token::Minus, '*' => Token::Asterisk, '/' => Token::Slash,
            '<' => Token::LessThan, '>' => Token::GreaterThan,
            '(' => Token::OpenParen, ')' => Token::CloseParen,
            '{' => Token::LBrace, '}' => Token::RBrace,
            ':' => Token::Colon, ',' => Token::Comma, '@' => Token::At,
            '\0' => Token::EOF,
            _ => {
                if self.is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return self.lookup_keyword_or_ident(&ident);
                } else if self.ch.is_ascii_digit() {
                    return Token::MoneyLiteral(self.read_number());
                } else {
                    Token::Illegal(self.ch)
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) || self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn lookup_keyword_or_ident(&self, ident: &str) -> Token {
        match ident {
            "asset" => Token::AssetKeyword, "if" => Token::If, "else" => Token::Else,
            "true" => Token::True, "false" => Token::False, "transaction" => Token::Transaction,
            _ => {
                // THE FIX: Forgiving currency check (allows 'kes', 'Usd' to become 'KES', 'USD')
                if ident.len() == 3 { Token::CurrencyTicker(ident.to_uppercase()) } 
                else { Token::Identifier(ident.to_string()) }
            }
        }
    }

    fn read_number(&mut self) -> Decimal {
        let position = self.position;
        // THE FIX: Securely allow commas inside accounting numbers!
        while self.ch.is_ascii_digit() || self.ch == '.' || self.ch == ',' {
            self.read_char();
        }
        let raw_str: String = self.input[position..self.position].iter().collect();
        let num_str = raw_str.replace(",", ""); // Strip commas before doing math
        num_str.parse::<Decimal>().unwrap_or(Decimal::ZERO)
    }

    fn is_letter(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read_char();
            } else if self.ch == '/' && self.peek_char() == '/' {
                while self.ch != '\n' && self.ch != '\0' { self.read_char(); }
            } else { break; }
        }
    }
}