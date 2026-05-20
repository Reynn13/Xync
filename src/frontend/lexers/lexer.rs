/*
    Head: Lexer main file
    Description: The declaration and implementation of Lexer

    @author LightMayo
*/

use crate::{Diagnostic, DiagnosticBuilder, Token, TokenTag, Tokens, XynError};

pub struct Lexer<'src> {
    src: &'src str,
    idx: usize,
}

pub(crate) enum LexerError {
    UnknownChar,
    UnclosedString,
    InvalidEscapeSequence,
    UnclosedMultiLineComment,
}

impl XynError for LexerError {
    fn to_usize(self) -> usize {
        self as usize
    }
    fn message(&self) -> &'static str {
        use LexerError::*;
        match self {
            UnknownChar => "Unknown character(s):",
            UnclosedString => "Unclosed string:",
            InvalidEscapeSequence => "Invalid escape sequence:",
            UnclosedMultiLineComment => "Unclosed multi line comment:",
        }
    }
}

impl<'src> Lexer<'src> {
    #[inline(always)]
    pub fn new(src: &'src str) -> Self {
        Self { src, idx: 0 }
    }

    #[inline(always)]
    fn get_c(&self) -> Option<char> {
        self.src.chars().nth(self.idx)
    }

    pub fn lex(&mut self) -> Result<Tokens, Vec<Diagnostic>> {
        let mut tokens = Tokens::with_capacity(self.src.len() / 3);
        let mut errors = Vec::new();

        while let Some(c) = self.get_c() {
            let start = self.idx;
            let kind = match c {
                '=' => {
                    self.idx += 1;
                    TokenTag::EqualOp
                }
                '+' => {
                    self.idx += 1;
                    TokenTag::PlusSign
                }
                '-' => {
                    self.idx += 1;
                    TokenTag::MinusSign
                }
                '*' => {
                    self.idx += 1;
                    TokenTag::StarSign
                }
                '/' => {
                    self.idx += 1;
                    TokenTag::DivSign
                }
                ';' => {
                    self.idx += 1;
                    TokenTag::SemicolonOp
                }
                '{' => {
                    self.idx += 1;
                    TokenTag::LCurlyBracket
                }
                '}' => {
                    self.idx += 1;
                    TokenTag::RCurlyBracket
                }
                // TODO: Handle newlines
                ch if ch.is_whitespace() => {
                    self.idx += 1;
                    continue;
                }
                '0'..='9' => {
                    self.idx += 1;
                    while let Some(c) = self.get_c() {
                        match c {
                            '0'..='9' => self.idx += 1,

                            _ => break,
                        }
                    }
                    TokenTag::Integer
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.idx += 1;
                    while let Some(c) = self.get_c() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.idx += 1,
                            _ => break,
                        }
                    }
                    match unsafe { self.src.get_unchecked(start..self.idx) } {
                        "let" => TokenTag::LetKeyword,
                        "mut" => TokenTag::MutKeyword,
                        "const" => TokenTag::ConstKeyword,
                        _ => TokenTag::Identifier,
                    }
                }
                '~' => {
                    self.idx += 1;
                    match self.get_c() {
                        Some('~') => {
                            // Found eof is just the same thing as `\n`
                            while let Some(c) = self.get_c() {
                                self.idx += 1;
                                if c == '\n' {
                                    break;
                                }
                            }
                            continue;
                        }
                        Some('*') => {
                            let mut closed = false;
                            while let Some(c) = self.get_c() {
                                self.idx += 1;
                                if c == '*' && self.get_c() == Some('~') {
                                    self.idx += 1;
                                    closed = true;
                                    break;
                                }
                            }
                            if !closed {
                                errors.push(
                                    DiagnosticBuilder::default()
                                        .with_error_code(LexerError::UnclosedMultiLineComment)
                                        .build(),
                                );
                            }
                            continue;
                        }
                        _ => {
                            // skip for now
                            continue;
                        }
                    }
                }
                '"' => {
                    self.idx += 1;
                    let mut buf = String::new();
                    let mut closed = false;
                    while let Some(c) = self.get_c() {
                        match c {
                            '\\' => {
                                self.idx += 1;
                                match self.get_c() {
                                    Some('"') => buf.push('"'),
                                    Some('\\') => buf.push('\\'),
                                    Some('n') => buf.push('n'),
                                    None => break,
                                    Some(_) => errors.push(
                                        DiagnosticBuilder::default()
                                            .with_error_code(LexerError::InvalidEscapeSequence)
                                            .build(),
                                    ),
                                }
                                self.idx += 1;
                            }
                            '"' => {
                                self.idx += 1;
                                closed = true;
                                break;
                            }
                            c => {
                                buf.push(c);
                                self.idx += 1;
                            }
                        }
                    }
                    // In here, `closed` will be still false
                    // if only when it checks for another closing `"`
                    // it found an EOF instead
                    if !closed {
                        errors.push(
                            DiagnosticBuilder::default()
                                .with_error_code(LexerError::UnclosedString)
                                .build(),
                        );
                    } else {
                        tokens.push(Token::new(buf), TokenTag::Str);
                    }
                    continue;
                }
                _ => {
                    errors.push(
                        DiagnosticBuilder::default()
                            // "0000" : Indicate an unknown symbol error
                            .with_error_code(LexerError::UnknownChar)
                            .build(),
                    );
                    self.idx += 1;
                    continue;
                }
            };

            let lexeme = unsafe { self.src.get_unchecked(start..self.idx) }.to_string();
            tokens.push(Token::new(lexeme), kind);
        }
        if errors.is_empty() {
            Ok(tokens)
        } else {
            // ! TODO: Error "0000" grouping
            Err(errors)
        }
    }
}
