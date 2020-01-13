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

enum Token {
    Keyword(Keyword),
    Identifier,
    Constant,
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

fn match_token(token_str: &'static str) -> Token {
    match token_str {
        /*
         * Keywords
         */
        // Storage class specifiers
        "auto" => Token::Keyword(Keyword::Auto),
        "extern" => Token::Keyword(Keyword::Extern),
        "register" => Token::Keyword(Keyword::Register),
        "static" => Token::Keyword(Keyword::Static),
        "typedef" => Token::Keyword(Keyword::Typedef),

        // Type qualifiers
        "const" => Token::Keyword(Keyword::Const),
        "restrict" => Token::Keyword(Keyword::Restrict),
        "volatile" => Token::Keyword(Keyword::Volatile),

        // Control
        "break" => Token::Keyword(Keyword::Break),
        "case" => Token::Keyword(Keyword::Case),
        "continue" => Token::Keyword(Keyword::Continue),
        "default" => Token::Keyword(Keyword::Default),
        "do" => Token::Keyword(Keyword::Do),
        "else" => Token::Keyword(Keyword::Else),
        "for" => Token::Keyword(Keyword::For),
        "goto" => Token::Keyword(Keyword::Goto),
        "if" => Token::Keyword(Keyword::If),
        "return" => Token::Keyword(Keyword::Return),
        "switch" => Token::Keyword(Keyword::Switch),
        "while" => Token::Keyword(Keyword::While),

        // Type specifiers
        "char" => Token::Keyword(Keyword::Char),
        "double" => Token::Keyword(Keyword::Double),
        "float" => Token::Keyword(Keyword::Float),
        "int" => Token::Keyword(Keyword::Int),
        "long" => Token::Keyword(Keyword::Long),
        "short" => Token::Keyword(Keyword::Short),
        "signed" => Token::Keyword(Keyword::Signed),
        "unsigned" => Token::Keyword(Keyword::Unsigned),
        "void" => Token::Keyword(Keyword::Void),

        // Struct, union, enumeration
        "enum" => Token::Keyword(Keyword::Enum),
        "struct" => Token::Keyword(Keyword::Struct),
        "union" => Token::Keyword(Keyword::Union),

        // Function specifiers
        "inline" => Token::Keyword(Keyword::Inline),

        // sizeof
        "sizeof" => Token::Keyword(Keyword::Sizeof),

        /*
         * Punctuators
         */
        "[" => Token::Punctuator(Punctuator::OpenSquare),
        "]" => Token::Punctuator(Punctuator::CloseSquare),
        "(" => Token::Punctuator(Punctuator::OpenParen),
        ")" => Token::Punctuator(Punctuator::CloseParen),
        "{" => Token::Punctuator(Punctuator::OpenBrace),
        "}" => Token::Punctuator(Punctuator::CloseBrace),

        "." => Token::Punctuator(Punctuator::Period),
        "->" => Token::Punctuator(Punctuator::Arrow),
        "++" => Token::Punctuator(Punctuator::PlusPlus),
        "--" => Token::Punctuator(Punctuator::MinusMinus),

        "&" => Token::Punctuator(Punctuator::Amp),
        "*" => Token::Punctuator(Punctuator::Asterisk),
        "+" => Token::Punctuator(Punctuator::Plus),
        "-" => Token::Punctuator(Punctuator::Minus),
        "~" => Token::Punctuator(Punctuator::Tilde),
        "!" => Token::Punctuator(Punctuator::Exclamation),
        "/" => Token::Punctuator(Punctuator::Slash),
        "%" => Token::Punctuator(Punctuator::Percent),

        "<<" => Token::Punctuator(Punctuator::LessLess),
        ">>" => Token::Punctuator(Punctuator::GreaterGreater),
        "<" => Token::Punctuator(Punctuator::Less),
        ">" => Token::Punctuator(Punctuator::Greater),
        "<=" => Token::Punctuator(Punctuator::LessEqual),
        ">=" => Token::Punctuator(Punctuator::GreaterEqual),
        "==" => Token::Punctuator(Punctuator::EqualEqual),
        "!=" => Token::Punctuator(Punctuator::ExclamationEqual),

        "^" => Token::Punctuator(Punctuator::Caret),
        "|" => Token::Punctuator(Punctuator::Pipe),
        "&&" => Token::Punctuator(Punctuator::AmpAmp),
        "||" => Token::Punctuator(Punctuator::PipePipe),
        "?" => Token::Punctuator(Punctuator::Question),
        ":" => Token::Punctuator(Punctuator::Colon),
        ";" => Token::Punctuator(Punctuator::SemiColon),
        "..." => Token::Punctuator(Punctuator::Elipsis),

        "=" => Token::Punctuator(Punctuator::Equal),
        "*=" => Token::Punctuator(Punctuator::AsteriskEqual),
        "/=" => Token::Punctuator(Punctuator::SlashEqual),
        "%=" => Token::Punctuator(Punctuator::PercentEqual),
        "+=" => Token::Punctuator(Punctuator::PlusEqual),
        "-=" => Token::Punctuator(Punctuator::MinusEqual),
        "<<=" => Token::Punctuator(Punctuator::LessLessEqual),
        ">>=" => Token::Punctuator(Punctuator::GreaterGreaterEqual),
        "&=" => Token::Punctuator(Punctuator::AmpEqual),
        "^=" => Token::Punctuator(Punctuator::CaretEqual),
        "|=" => Token::Punctuator(Punctuator::PipeEqual),

        "," => Token::Punctuator(Punctuator::Comma),
        "#" => Token::Punctuator(Punctuator::Hash),
        "##" => Token::Punctuator(Punctuator::HashHash),

        // Digraphs
        "<:" => Token::Punctuator(Punctuator::OpenSquare),
        ":>" => Token::Punctuator(Punctuator::CloseSquare),
        "<%" => Token::Punctuator(Punctuator::OpenBrace),
        "%>" => Token::Punctuator(Punctuator::CloseBrace),
        "%:" => Token::Punctuator(Punctuator::Hash),
        "%:%:" => Token::Punctuator(Punctuator::HashHash),
    }
}

fn lex(program_str: &'static str) -> Vec<Token> {
    // FSM through characters of program
    return vec![match_token("}")];
}
