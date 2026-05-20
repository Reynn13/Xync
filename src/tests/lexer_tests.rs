#[cfg(test)]
pub mod lexer_tests {
    use crate::{DiagnosticBuilder, Lexer, LexerError, Token, TokenTag, Tokens};

    #[test]
    fn unknown_chars() {
        let mut lexer = Lexer::new("## ` @");
        assert_eq!(
            lexer.lex(),
            Err(vec![
                DiagnosticBuilder::default()
                    .with_error_code(LexerError::UnknownChar)
                    .build(),
                DiagnosticBuilder::default()
                    .with_error_code(LexerError::UnknownChar)
                    .build(),
                DiagnosticBuilder::default()
                    .with_error_code(LexerError::UnknownChar)
                    .build(),
                DiagnosticBuilder::default()
                    .with_error_code(LexerError::UnknownChar)
                    .build()
            ])
        );
    }

    #[test]
    pub fn multi_whitespaces() {
        let mut lexer = Lexer::new(
            "let    num
=
100   ;


{       }",
        );
        let mut expected_tokens = Tokens::new();
        expected_tokens.push(Token::new("let"), TokenTag::LetKeyword);
        expected_tokens.push(Token::new("num"), TokenTag::Identifier);
        expected_tokens.push(Token::new("="), TokenTag::EqualOp);
        expected_tokens.push(Token::new("100"), TokenTag::Integer);
        expected_tokens.push(Token::new(";"), TokenTag::SemicolonOp);
        expected_tokens.push(Token::new("{"), TokenTag::LCurlyBracket);
        expected_tokens.push(Token::new("}"), TokenTag::RCurlyBracket);
        assert_eq!(lexer.lex(), Ok(expected_tokens));
    }

    #[test]
    fn unclosed_string() {
        let mut lexer = Lexer::new("let a = \"hello\" + \"world");
        assert_eq!(
            lexer.lex(),
            Err(vec![
                DiagnosticBuilder::default()
                    .with_error_code(LexerError::UnclosedString)
                    .build()
            ])
        )
    }

    #[test]
    fn unclosed_comments() {
        let mut lexer = Lexer::new("let a = \"hello\"; ~* unterminated");
        assert_eq!(
            lexer.lex(),
            Err(vec![
                DiagnosticBuilder::default()
                    .with_error_code(LexerError::UnclosedMultiLineComment)
                    .build()
            ])
        )
    }

    #[test]
    fn escaped_string() {
        let mut lexer = Lexer::new("\"My name is \\\"John\\\"!\"");
        let mut expected_tokens = Tokens::new();
        expected_tokens.push(Token::new("My name is \"John\"!"), TokenTag::Str);
        assert_eq!(lexer.lex(), Ok(expected_tokens))
    }
}
