/*
    Head: Lexer Token
    Description: Lexer Token's APIs and Items

    @author LightMayo
*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenTag {
    Identifier,
    Integer,
    Str,

    EqualOp,
    SemicolonOp,
    LCurlyBracket,
    RCurlyBracket,

    PlusSign,
    MinusSign,
    StarSign,
    DivSign,

    LetKeyword,
    MutKeyword,
    ConstKeyword,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub lexeme: String,
}

impl Token {
    pub fn new(lexeme: impl ToString) -> Self {
        Self { lexeme: lexeme.to_string() }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tokens {
    tokens: Vec<Token>,
    kinds: Vec<TokenTag>,
}

impl Tokens {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tokens: Vec::with_capacity(capacity),
            kinds: Vec::with_capacity(capacity),
        }
    }
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            kinds: Vec::new(),
        }
    }

    pub fn push(&mut self, token: Token, kind: TokenTag) {
        self.tokens.push(token);
        self.kinds.push(kind);
    }

    pub fn get_token(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(idx)
    }
    pub fn get_kind(&self, idx: usize) -> Option<&TokenTag> {
        self.kinds.get(idx)
    }
}
