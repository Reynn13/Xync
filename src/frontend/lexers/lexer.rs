use crate::frontend::{
    Interner,
    token::{Token, TokenKind, TokenLocation},
};

pub struct Lexer<'src> {
    src: &'src [u8],
    idx: usize,
    col: u16,
    line: u16,
    len: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src [u8]) -> Self {
        let len = src.len();
        Self {
            src,
            idx: 0,
            col: 1,
            line: 1,
            len,
        }
    }
    fn get_str(&self, start_idx: usize) -> &str {
        str::from_utf8(self.src)
            .unwrap()
            .get(start_idx..self.idx)
            .unwrap()
    }
    pub fn lex(&mut self, interner: &mut Interner) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.idx < self.len {
            let start_idx = self.idx;
            let start_col = self.col;
            let start_line = self.line;

            self.idx += 1;
            self.col += 1;
            let kind = match self.src[start_idx] {
                b'\n' => {
                    self.line += 1;
                    self.col = 1;
                    continue;
                }
                b' ' | b'\t' | b'\r' => continue,

                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    while matches!(self.src.get(self.idx), Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')) {
                        self.idx += 1;
                        self.col += 1;
                    }

                    TokenKind::ValueIdentifier
                }
                b'0'..=b'9' => {
                    while matches!(self.src.get(self.idx), Some(b'0'..=b'9')) {
                        self.idx += 1;
                        self.col += 1;
                    }
                    TokenKind::ValueInteger
                }

                b'=' => TokenKind::OpEqual,
                b';' => TokenKind::OpSemicolon,

                b'+' => TokenKind::OpAdd,
                b'-' => TokenKind::OpSub,
                b'*' => TokenKind::OpMul,
                b'/' => {
                    // ? Single comment
                    if let Some(b'/') = self.src.get(self.idx) {
                        // ? Skip until found newline or eof
                        while !matches!(self.src.get(self.idx), Some(b'\n') | None) {
                            self.idx += 1;
                        }
                        continue;
                    }
                    TokenKind::OpDiv
                }

                // ! TODO: Change this into diagnostic
                _ => continue
            };

            let lexeme_id = interner.intern(self.get_str(start_idx));

            // ? This valid if using the `.with_predefined_keywords` function
            let kind = match lexeme_id.0 {
                0 => TokenKind::KeywordLet,
                1 => TokenKind::KeywordMut,
                _ => kind,
            };
            let location = TokenLocation {
                start_idx,
                end_idx: self.idx,
                start_col,
                end_col: self.col,
                start_line,
                end_line: self.line
            };

            tokens.push(Token { kind, lexeme_id, location})
        }
        tokens
    }
}
