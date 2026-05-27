use std::fmt::Display;

use crate::frontend::StringId;

#[derive(PartialEq)]
pub enum TokenKind {
    KeywordLet,
    KeywordMut,
    
    OpEqual,
    OpSemicolon,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,

    ValueIdentifier,
    ValueInteger
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        write!(f, "Kind: `{}`", match self {
            KeywordLet => "Let",
            KeywordMut => "Mut",


            OpEqual => "Equal",
            OpSemicolon => "Semicolon",
            
            OpAdd => "Add",
            OpSub => "Sub",
            OpMul => "Mul",
            OpDiv => "Div",


            ValueIdentifier => "Identifier",
            ValueInteger => "Integer"
        })
    }
}

pub struct TokenLocation {
    pub start_idx: usize,
    pub end_idx: usize,
    pub start_col: u16,
    pub end_col: u16,
    pub start_line: u16,
    pub end_line: u16,
}

impl Display for TokenLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Location [idx: {}-{}, col: {}-{}, line: {}-{}]",
            self.start_idx,
            self.end_idx,
            self.start_col,
            self.end_col,
            self.start_line,
            self.end_line
        )
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub lexeme_id: StringId,
    pub location: TokenLocation,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token:\n  - String Id: {}\n  - {}\n  - {}", self.lexeme_id.0, self.kind, self.location)
    }
}
