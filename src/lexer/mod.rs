pub mod token;
use token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,      // Current character position
    read_position: usize, // Next character position
    ch: char,             // The actual character we are looking at
}

impl Lexer {
    // 1. Initialize the Lexer with source code
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

    // 2. Move forward one character safely
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // End of file
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // 3. The Core Logic: Match characters to Tokens
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::Assign,
            '\0' => Token::EOF,
            _ => {
                if self.is_letter(self.ch) {
                    let ident = self.read_identifier();
                    // If it's a 3-letter uppercase word, treat it as a Currency
                    if ident.len() == 3 && ident.chars().all(|c| c.is_uppercase()) {
                        return Token::CurrencyTicker(ident);
                    }
                    return Token::Identifier(ident);
                } else if self.ch.is_ascii_digit() {
                    let num = self.read_number();
                    return Token::MoneyLiteral(num);
                } else {
                    Token::Illegal(self.ch)
                }
            }
        };

        self.read_char();
        tok
    }

    // Helper: Read a full word
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }
    
    // Helper: Read a number
    fn read_number(&mut self) -> f64 {
        let position = self.position;
        while self.ch.is_ascii_digit() || self.ch == '.' {
            self.read_char();
        }
        let num_str: String = self.input[position..self.position].iter().collect();
        num_str.parse::<f64>().unwrap_or(0.0) 
    }

    fn is_letter(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}