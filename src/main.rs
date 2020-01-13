enum Token {
    Keyword,
    Identifier,
    Constant,
    StringLiteral,
    Punctuator,
}

enum PreprocessingToken {
    HeaderName,
    Identifier,
    PpNumber,
    CharacterConstant,
    StringLiteral,
    Punctuator,
}

fn lex(token_str: &'static str) -> Token {
    match token_str {
        "auto" | "break" | "case" | "char"
            | "const" | "continue" | "default"
            | "do" | "double" | "else" | "enum"
            | "extern" | "float" | "for" | "goto"
            | "if" | "inline" | "int" | "long"
            | "register" | "restrict" | "return"
            | "short" | "signed" | "sizeof" | "static"
            | "struct" | "switch" | "typedef" | "union"
            | "unsigned" | "void" | "volatile" | "while" => Token::Keyword,

        "[" | "]" | "(" | ")" | "{" | "}" | "." | "->"
            | "++" | "--" | "&" | "*" | "+" | "-" | "~" | "!"
            | "/" | "%" | "<<" | ">>" | "<" | ">"
            | "<=" | ">=" | "==" | "!=" | "^" | "|" | "&&" | "||"
            | "?" | ":" | ";" | "..."
            | "=" | "*=" | "/=" | "%=" | "+=" | "-=" | "<<=" | ">>=" | "&=" | "^=" | "|="
            | "," | "#" | "##"
            | "<:" | ":>" | "<%" | "%>" | "%:" | "%:%:" => Token::Punctuator,
    }
}
