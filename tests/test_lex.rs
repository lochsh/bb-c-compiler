use compiler::lex::{Constant, Keyword, Lexer, LexerState, Punctuator, Token};
use std::fs;

#[test]
fn test_int_main_return_0() {
    let program_str = fs::read_to_string("tests/res/return0.c").unwrap();
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
        Token::Constant(Constant::Int(0)),
        Token::Punctuator(Punctuator::SemiColon),
        Token::Punctuator(Punctuator::CloseBrace),
    ];

    assert_eq!(&expected_tokens[..], &lexer.tokens()[..]);
}
