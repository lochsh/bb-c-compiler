#![allow(unused)]
mod tokens;
use tokens::PunctuatorCharResult;
pub use tokens::{Constant, Keyword, Punctuator, Token};

pub enum LexerState {
    NewToken,
    KeywordOrId,
    Punctuator,
    StringLiteral,
    Constant,
}

pub struct Lexer {
    accum: Vec<char>,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            accum: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn step(&mut self, state: LexerState, c: char) -> Result<LexerState, String> {
        match state {
            LexerState::NewToken => self.new_token(c),
            LexerState::KeywordOrId => self.accumulate_keyword_or_identifier(c),
            LexerState::Punctuator => self.accumulate_punctuator(c),
            LexerState::StringLiteral => self.accumulate_string(c),
            LexerState::Constant => self.accumulate_constant(c),
        }
    }

    fn accumulate_token_str(&mut self) -> String {
        let token_str: String = self.accum.iter().collect();
        self.accum.clear();
        token_str
    }

    fn new_token(&mut self, c: char) -> Result<LexerState, String> {
        if c.is_ascii_whitespace() {
            return Ok(LexerState::NewToken);
        }

        self.accum.push(c);

        if c.is_ascii_alphabetic() {
            Ok(LexerState::KeywordOrId)
        } else if c.is_ascii_digit() {
            Ok(LexerState::Constant)
        } else if Punctuator::from_char(&c) != PunctuatorCharResult::NoMatch {
            Ok(LexerState::Punctuator)
        } else if c == '"' {
            Ok(LexerState::StringLiteral)
        } else {
            Err(format!("State transition not implemented, character: {}", c))
        }
    }

    fn accumulate_keyword_or_identifier(&mut self, c: char) -> Result<LexerState, String> {
        if c.is_ascii_alphanumeric() {
            self.accum.push(c);
            return Ok(LexerState::KeywordOrId);
        }

        // Finished accumulating
        let token_str = self.accumulate_token_str();

        match Keyword::from_string(&token_str) {
            Some(x) => self.tokens.push(Token::Keyword(x)),
            None => self.tokens.push(Token::Identifier(token_str)),
        };

        self.new_token(c)
    }

    fn accumulate_punctuator(&mut self, c: char) -> Result<LexerState, String> {
        match Punctuator::from_char(&c) {
            PunctuatorCharResult::IncompleteToken => {
                self.accum.push(c);
                Ok(LexerState::Punctuator)
            }

            PunctuatorCharResult::CompleteToken(x) => {
                self.accum.clear();
                self.tokens.push(Token::Punctuator(x));
                Ok(LexerState::NewToken)
            }

            PunctuatorCharResult::NoMatch => {
                let token_str = self.accumulate_token_str();

                match Punctuator::from_string(&token_str) {
                    Some(x) => self.tokens.push(Token::Punctuator(x)),
                    None => panic!(format!("Bad sequence: {} is not a punctuator", token_str)),
                };

                self.new_token(c)
            }
        }
    }

    fn accumulate_string(&mut self, c: char) -> Result<LexerState, String> {
        match c {
            '"' => {
                let token_str = self.accumulate_token_str();
                self.tokens.push(Token::StringLiteral(token_str));
                Ok(LexerState::NewToken)
            }

            _ => {
                self.accum.push(c);
                Ok(LexerState::StringLiteral)
            }
        }
    }

    fn accumulate_constant(&mut self, c: char) -> Result<LexerState, String> {
        if c.is_ascii_digit() {
            Ok(LexerState::Constant)
        } else {
            // TODO deal with error instead of unwrapping?
            // TODO deal with non integer constants
            // TODO deal with hex and octal
            let token_str = self.accumulate_token_str();
            match token_str.parse::<i32>() {
                Ok(x) => {
                    self.tokens.push(Token::Constant(Constant::Int(x)));
                    self.new_token(c)
                }

                Err(s) => Err(format!("Unable to parse constant: {}", s)),
            }
        }
    }
}
