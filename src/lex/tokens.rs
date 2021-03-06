#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Constant(Constant),
    StringLiteral(String),
    Punctuator(Punctuator),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
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

impl Keyword {
    pub fn from_string(token_str: &str) -> Option<Keyword> {
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

#[derive(Debug, PartialEq)]
pub enum Punctuator {
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

#[derive(PartialEq)]
pub enum PunctuatorCharResult {
    CompleteToken(Punctuator),
    IncompleteToken,
    NoMatch,
}

impl Punctuator {
    pub fn from_string(token_str: &str) -> Option<Punctuator> {
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

    pub fn from_char(token_char: &char) -> PunctuatorCharResult {
        match token_char {
            '[' => PunctuatorCharResult::CompleteToken(Punctuator::OpenSquare),
            ']' => PunctuatorCharResult::CompleteToken(Punctuator::CloseSquare),
            '(' => PunctuatorCharResult::CompleteToken(Punctuator::OpenParen),
            ')' => PunctuatorCharResult::CompleteToken(Punctuator::CloseParen),
            '{' => PunctuatorCharResult::CompleteToken(Punctuator::OpenBrace),
            '}' => PunctuatorCharResult::CompleteToken(Punctuator::CloseBrace),
            ';' => PunctuatorCharResult::CompleteToken(Punctuator::SemiColon),
            ',' => PunctuatorCharResult::CompleteToken(Punctuator::Comma),

            '.' | '|' | ':' | '!' | '-' | '+' | '=' | '*' | '/' | '!' | '?' | '<' | '>' | '#'
            | '&' | '^' | '%' => PunctuatorCharResult::IncompleteToken,

            _ => PunctuatorCharResult::NoMatch,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Constant {
    Int(i32),
    Unsigned(u32),
    Long(i64),
    UnsignedLong(u64),
    Float(f32),
    Double(f64),
    Size(usize),
    Char(char),
}
