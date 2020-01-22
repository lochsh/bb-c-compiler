#![allow(unused)]

#[cfg(test)]
use std::fs;

#[derive(Debug, PartialEq)]
enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

#[derive(Debug, PartialEq)]
enum Punctuator {
    OpenSquare,
    CloseSquare,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Period,
    Arrow,
    PlusPlus,
    MinusMinus,
    Amp,
    Asterisk,
    Plus,
    Minus,
    Tilde,
    Exclamation,
    Slash,
    Percent,
    LessLess,
    GreaterGreater,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    EqualEqual,
    ExclamationEqual,
    Caret,
    Pipe,
    AmpAmp,
    PipePipe,
    Question,
    Colon,
    SemiColon,
    Elipsis,
    Equal,
    AsteriskEqual,
    SlashEqual,
    PercentEqual,
    PlusEqual,
    MinusEqual,
    LessLessEqual,
    GreaterGreaterEqual,
    AmpEqual,
    CaretEqual,
    PipeEqual,
    Comma,
    Hash,
    HashHash,
}

#[derive(Debug, PartialEq)]
enum Token {
    Keyword(Keyword),
    Identifier(String),
    Constant(usize),  // TODO handle non-integer constants
    StringLiteral(String),
    Punctuator(Punctuator),
}

enum PreprocessingToken {
    HeaderName,
    Identifier,
    PpNumber,
    CharacterConstant,
    StringLiteral,
    Punctuator,
}

impl Keyword {
    fn from_string(token_str: &str) -> Option<Keyword> {
        match token_str {
            // Storage class specifiers
            "auto" => Some(Keyword::Auto),
            "extern" => Some(Keyword::Extern),
            "register" => Some(Keyword::Register),
            "static" => Some(Keyword::Static),
            "typedef" => Some(Keyword::Typedef),

            // Type qualifiers
            "const" => Some(Keyword::Const),
            "restrict" => Some(Keyword::Restrict),
            "volatile" => Some(Keyword::Volatile),

            // Control
            "break" => Some(Keyword::Break),
            "case" => Some(Keyword::Case),
            "continue" => Some(Keyword::Continue),
            "default" => Some(Keyword::Default),
            "do" => Some(Keyword::Do),
            "else" => Some(Keyword::Else),
            "for" => Some(Keyword::For),
            "goto" => Some(Keyword::Goto),
            "if" => Some(Keyword::If),
            "return" => Some(Keyword::Return),
            "switch" => Some(Keyword::Switch),
            "while" => Some(Keyword::While),

            // Type specifiers
            "char" => Some(Keyword::Char),
            "double" => Some(Keyword::Double),
            "float" => Some(Keyword::Float),
            "int" => Some(Keyword::Int),
            "long" => Some(Keyword::Long),
            "short" => Some(Keyword::Short),
            "signed" => Some(Keyword::Signed),
            "unsigned" => Some(Keyword::Unsigned),
            "void" => Some(Keyword::Void),

            // Struct, union, enumeration
            "enum" => Some(Keyword::Enum),
            "struct" => Some(Keyword::Struct),
            "union" => Some(Keyword::Union),

            // Function specifiers
            "inline" => Some(Keyword::Inline),

            // sizeof
            "sizeof" => Some(Keyword::Sizeof),

            // Failed to match keyword
            _ => None,
        }
    }
}

impl Punctuator {
    fn from_string(token_str: &str) -> Option<Punctuator> {
        match token_str {
            "[" => Some(Punctuator::OpenSquare),
            "]" => Some(Punctuator::CloseSquare),
            "(" => Some(Punctuator::OpenParen),
            ")" => Some(Punctuator::CloseParen),
            "{" => Some(Punctuator::OpenBrace),
            "}" => Some(Punctuator::CloseBrace),

            "." => Some(Punctuator::Period),
            "->" => Some(Punctuator::Arrow),
            "++" => Some(Punctuator::PlusPlus),
            "--" => Some(Punctuator::MinusMinus),

            "&" => Some(Punctuator::Amp),
            "*" => Some(Punctuator::Asterisk),
            "+" => Some(Punctuator::Plus),
            "-" => Some(Punctuator::Minus),
            "~" => Some(Punctuator::Tilde),
            "!" => Some(Punctuator::Exclamation),
            "/" => Some(Punctuator::Slash),
            "%" => Some(Punctuator::Percent),

            "<<" => Some(Punctuator::LessLess),
            ">>" => Some(Punctuator::GreaterGreater),
            "<" => Some(Punctuator::Less),
            ">" => Some(Punctuator::Greater),
            "<=" => Some(Punctuator::LessEqual),
            ">=" => Some(Punctuator::GreaterEqual),
            "==" => Some(Punctuator::EqualEqual),
            "!=" => Some(Punctuator::ExclamationEqual),

            "^" => Some(Punctuator::Caret),
            "|" => Some(Punctuator::Pipe),
            "&&" => Some(Punctuator::AmpAmp),
            "||" => Some(Punctuator::PipePipe),
            "?" => Some(Punctuator::Question),
            ":" => Some(Punctuator::Colon),
            ";" => Some(Punctuator::SemiColon),
            "..." => Some(Punctuator::Elipsis),

            "=" => Some(Punctuator::Equal),
            "*=" => Some(Punctuator::AsteriskEqual),
            "/=" => Some(Punctuator::SlashEqual),
            "%=" => Some(Punctuator::PercentEqual),
            "+=" => Some(Punctuator::PlusEqual),
            "-=" => Some(Punctuator::MinusEqual),
            "<<=" => Some(Punctuator::LessLessEqual),
            ">>=" => Some(Punctuator::GreaterGreaterEqual),
            "&=" => Some(Punctuator::AmpEqual),
            "^=" => Some(Punctuator::CaretEqual),
            "|=" => Some(Punctuator::PipeEqual),

            "," => Some(Punctuator::Comma),
            "#" => Some(Punctuator::Hash),
            "##" => Some(Punctuator::HashHash),

            // Digraphs
            "<:" => Some(Punctuator::OpenSquare),
            ":>" => Some(Punctuator::CloseSquare),
            "<%" => Some(Punctuator::OpenBrace),
            "%>" => Some(Punctuator::CloseBrace),
            "%:" => Some(Punctuator::Hash),
            "%:%:" => Some(Punctuator::HashHash),

            // Trigraphs
            "??=" => Some(Punctuator::Hash),
            "??/" => Some(Punctuator::Slash),
            "??'" => Some(Punctuator::Caret),
            "??(" => Some(Punctuator::OpenSquare),
            "??)" => Some(Punctuator::CloseSquare),
            "??!" => Some(Punctuator::Pipe),
            "??<" => Some(Punctuator::OpenBrace),
            "??>" => Some(Punctuator::CloseBrace),
            "??-" => Some(Punctuator::Tilde),

            // Failed to match punctuator
            _ => None,
        }
    }

    fn from_char(token_char: &char) -> PunctuatorCharResult {
        match token_char {
            '[' => PunctuatorCharResult::CompleteToken(Punctuator::OpenSquare),
            ']' => PunctuatorCharResult::CompleteToken(Punctuator::CloseSquare),
            '(' => PunctuatorCharResult::CompleteToken(Punctuator::OpenParen),
            ')' => PunctuatorCharResult::CompleteToken(Punctuator::CloseParen),
            '{' => PunctuatorCharResult::CompleteToken(Punctuator::OpenBrace),
            '}' => PunctuatorCharResult::CompleteToken(Punctuator::CloseBrace),
            ';' => PunctuatorCharResult::CompleteToken(Punctuator::SemiColon),
            ',' => PunctuatorCharResult::CompleteToken(Punctuator::Comma),

            '.' | '|' | ':' | '!' |
            '-' | '+' | '=' | '*' | '/' |
            '!' | '?' | '<' | '>' | '#' |
            '&' | '|' | '^' | '%' => PunctuatorCharResult::IncompleteToken,

            _ => PunctuatorCharResult::NoMatch,
        }
    }
}

#[derive(PartialEq)]
enum PunctuatorCharResult {
    CompleteToken(Punctuator),
    IncompleteToken,
    NoMatch,
}


enum LexerState {
    NewToken,
    KeywordOrId,
    Punctuator,
    StringLiteral,
    Constant,
}

struct Lexer {
    accum: Vec<char>,
    tokens: Vec<Token>,
}

impl Lexer {

    fn new() -> Self {
        Lexer {
            accum: Vec::new(),
            tokens: Vec::new(),
        }
    }

    fn step(&mut self, state: LexerState, c: char) -> LexerState {
        match state {
            LexerState::NewToken => self.new_token(c),
            LexerState::KeywordOrId=> self.accumulate_keyword_or_identifier(c),
            LexerState::Punctuator=> self.accumulate_punctuator(c),
            LexerState::StringLiteral => self.accumulate_string(c),
            LexerState::Constant => self.accumulate_constant(c),
        }
    }

    fn accumulate_token_str(&mut self) -> String {
        let token_str: String = self.accum.iter().collect();
        self.accum.clear();
        token_str
    }

    fn new_token(&mut self, c: char) -> LexerState {
        if c.is_ascii_whitespace() {
            return LexerState::NewToken;
        }

        self.accum.push(c);

        if c.is_ascii_alphabetic() {
            return LexerState::KeywordOrId;
        }

        if c.is_ascii_digit() {
            return LexerState::Constant;
        }

        if Punctuator::from_char(&c) != PunctuatorCharResult::NoMatch {
            return LexerState::Punctuator;
        }

        if c == '"' {
            return LexerState::StringLiteral;
        }

        panic!(format!("State transition not implemented, character: {}", c));
    }

    fn accumulate_keyword_or_identifier(&mut self, c: char) -> LexerState {
        if c.is_ascii_alphanumeric() {
            self.accum.push(c);
            return LexerState::KeywordOrId
        }

        // Finished accumulating
        let token_str = self.accumulate_token_str();

        match Keyword::from_string(&token_str) {
            Some(x) => self.tokens.push(Token::Keyword(x)),
            None => self.tokens.push(Token::Identifier(token_str)),
        };

        self.new_token(c)
    }

    fn accumulate_punctuator(&mut self, c: char) -> LexerState {
        match Punctuator::from_char(&c) {
            PunctuatorCharResult::IncompleteToken => {
                self.accum.push(c);
                LexerState::Punctuator
            },

            PunctuatorCharResult::CompleteToken(x) => {
                self.accum.clear();
                self.tokens.push(Token::Punctuator(x));
                LexerState::NewToken
            },

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

    fn accumulate_string(&mut self, c: char) -> LexerState {
        match c {
            '"' => {
                let token_str = self.accumulate_token_str();
                self.tokens.push(Token::StringLiteral(token_str));
                LexerState::NewToken
            }

            _ => {
                self.accum.push(c);
                LexerState::StringLiteral
            },
        }
    }

    fn accumulate_constant(&mut self, c: char) -> LexerState {
        if c.is_ascii_digit() {
            LexerState::Constant
        } else {
            // TODO deal with error instead of unwrapping?
            // TODO deal with non integer constants
            let token_str = self.accumulate_token_str();
            self.tokens.push(Token::Constant(token_str.parse::<usize>().unwrap()));
            self.new_token(c)
        }
    }
}

#[test]
fn test_int_main_return_0() {
    let program_str = fs::read_to_string("test/res/return0.c").unwrap();
    let mut lexer = Lexer::new();
    let mut state = LexerState::NewToken;

    for c in program_str.chars() {
        state = lexer.step(state, c);
    }

    let expected_tokens = vec![
        Token::Keyword(Keyword::Int),
        Token::Identifier("main".to_string()),
        Token::Punctuator(Punctuator::OpenParen),
        Token::Keyword(Keyword::Void),
        Token::Punctuator(Punctuator::CloseParen),
        Token::Punctuator(Punctuator::OpenBrace),
        Token::Keyword(Keyword::Return),
        Token::Constant(0),
        Token::Punctuator(Punctuator::SemiColon),
        Token::Punctuator(Punctuator::CloseBrace)];

    assert_eq!(&expected_tokens[..], &lexer.tokens[..]);
}
