/*
    Head: Parser main file
    Description: The declaration and implementation of Parser

    @author LightMayo
*/

use crate::{
    Diagnostic, DiagnosticBuilder, Mutability, OpKind, Scope, ScopeArena, ScopeId, ScopeType,
    Statement, Token, TokenTag, Tokens, Value, Variable, VariableDescriptor, XynError,
};

pub struct Parser {
    tokens: Tokens,
    idx: usize,
}

pub(crate) enum ParserError {
    InvalidStartOfStatement,
    ExpectedEqual,
    ExpectedIdentifier,
    ExpectedSemicolon,
    ExpectedExpr,
    InvalidExpr,
    UnclosedScope,
}

impl XynError for ParserError {
    fn to_usize(self) -> usize {
        1000 + self as usize
    }
    fn message(&self) -> &'static str {
        use ParserError::*;
        match self {
            InvalidStartOfStatement => "Invalid start of statement:",
            ExpectedEqual => "Expected an equal sign:",
            ExpectedIdentifier => "Expected an identifier:",
            ExpectedSemicolon => "Expected a semicolon:",
            ExpectedExpr => "Expected an expr:",
            InvalidExpr => "Found an invalid expr:",
            UnclosedScope => "Unclosed scope:",
        }
    }
}

impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self { tokens, idx: 0 }
    }

    fn get_token(&self) -> Option<Token> {
        self.tokens.get_token(self.idx).cloned()
    }
    fn next_token(&mut self) -> Option<Token> {
        let tok = self.tokens.get_token(self.idx).cloned();
        self.idx += 1;
        tok
    }
    fn get_kind(&self) -> TokenTag {
        self.tokens
            .get_kind(self.idx)
            .cloned()
            .unwrap_or(TokenTag::EOF)
    }
    fn skip_to(&mut self, checkpoint: TokenTag) {
        loop {
            let tok = self.get_kind();
            if tok == TokenTag::EOF || tok == checkpoint {
                break;
            }
            self.idx += 1;
        }
    }
    fn skip_to_many(&mut self, checkpoints: &[TokenTag]) -> TokenTag {
        loop {
            let tok = self.get_kind();
            if tok == TokenTag::EOF || checkpoints.contains(&tok) {
                return tok;
            }
            self.idx += 1;
        }
    }
    fn expect(
        &mut self,
        expected: TokenTag,
        checkpoint: TokenTag,
        error: ParserError,
    ) -> Option<Diagnostic> {
        if self.get_kind() != expected {
            self.skip_to(checkpoint);
            Some(DiagnosticBuilder::default().with_error_code(error).build())
        } else {
            self.idx += 1;
            None
        }
    }
    fn expect_branch(
        &mut self,
        expected: &[TokenTag],
        checkpoint: &[TokenTag],
        error: ParserError,
    ) -> Result<TokenTag, (Diagnostic, TokenTag)> {
        let kind = self.get_kind().clone();
        if expected.contains(&kind) {
            self.idx += 1;
            Ok(kind)
        } else {
            let kind = self.skip_to_many(checkpoint);
            Err((
                DiagnosticBuilder::default().with_error_code(error).build(),
                kind,
            ))
        }
    }

    // ? NOTE: Every scope in a file will always be a children to the file_scope
    fn parse_scope(
        &mut self,
        valid_statements: &Vec<TokenTag>,
        errors: &mut Vec<Diagnostic>,
        scope_arena: &mut ScopeArena,
        scope_id_counter: &mut ScopeId,
    ) {
        self.idx += 1; // skip '{'
        let scope = Scope::with_parent(Some(*scope_id_counter), ScopeType::Local);
        scope_arena.add_scope(scope);
        *scope_id_counter += 1;

        self.inner_parse(
            valid_statements,
            errors,
            scope_arena,
            scope_id_counter,
            &[TokenTag::EOF, TokenTag::RCurlyBracket],
        );
        match self.get_kind() {
            TokenTag::RCurlyBracket => self.idx += 1,
            TokenTag::EOF => errors.push(
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::UnclosedScope)
                    .build(),
            ),
            _ => unreachable!(),
        }
    }

    fn inner_parse(
        &mut self,
        valid_statements: &Vec<TokenTag>,
        errors: &mut Vec<Diagnostic>,
        scope_arena: &mut ScopeArena,
        scope_id_counter: &mut ScopeId,
        breaks: &[TokenTag],
    ) {
        loop {
            let tok = self.get_kind();
            if let Some(stmt) = match tok {
                tok if breaks.contains(&tok) => break,

                // ? start of scope
                TokenTag::LCurlyBracket => {
                    self.parse_scope(valid_statements, errors, scope_arena, scope_id_counter);
                    continue;
                }

                TokenTag::LetKeyword => {
                    self.parse_var(errors, VariableDescriptor::default(Mutability::Immutable))
                }
                TokenTag::MutKeyword => {
                    self.parse_var(errors, VariableDescriptor::default(Mutability::Mutable))
                }
                TokenTag::ConstKeyword => {
                    self.parse_var(errors, VariableDescriptor::default(Mutability::Const))
                }

                TokenTag::SemicolonOp => {
                    self.idx += 1;
                    continue;
                }

                _ => {
                    errors.push(
                        DiagnosticBuilder::default()
                            .with_error_code(ParserError::InvalidStartOfStatement)
                            .build(),
                    );
                    self.skip_to_many(valid_statements);
                    continue;
                }
            } {
                scope_arena.add_statement_to(*scope_id_counter, stmt);
            }
        }
    }

    pub fn parse(&mut self) -> Result<ScopeArena, Vec<Diagnostic>> {
        let mut errors = Vec::new();
        let mut scope_arena = ScopeArena::new();
        let file_scope = Scope::new(ScopeType::Local);
        scope_arena.add_scope(file_scope);
        let valid_statements = vec![
            TokenTag::LetKeyword,
            TokenTag::MutKeyword,
            TokenTag::ConstKeyword,
        ];
        let mut scope_id_counter = 0;
        self.inner_parse(
            &valid_statements,
            &mut errors,
            &mut scope_arena,
            &mut scope_id_counter, // file_scope
            &[TokenTag::EOF],
        );
        if errors.is_empty() {
            Ok(scope_arena)
        } else {
            Err(errors)
        }
    }
}

impl Parser {
    fn parse_prefix(&mut self) -> Result<Value, Diagnostic> {
        match self.get_kind() {
            TokenTag::Integer => Ok(Value::Integer(self.next_token().unwrap().lexeme)),
            TokenTag::Identifier => Ok(Value::Identifier(self.next_token().unwrap().lexeme.clone())),
            TokenTag::PlusSign => {
                self.idx += 1;
                Ok(Value::Absolute(match self.parse_prefix() {
                    Ok(val) => match val {
                        Value::Absolute(value) | Value::Negate(value) => value, // simplify recursive absolutes
                        value => Box::new(value),
                    },
                    Err(error) => return Err(error),
                }))
            }
            TokenTag::MinusSign => {
                self.idx += 1;
                Ok(Value::Negate(match self.parse_prefix() {
                    Ok(val) => match val {
                        Value::Negate(value) => return Ok(*value), // simplify recursive negates
                        value => Box::new(value),
                    },
                    Err(error) => return Err(error),
                }))
            }
            TokenTag::SemicolonOp => Err(DiagnosticBuilder::default()
                .with_error_code(ParserError::ExpectedExpr)
                .build()),
            _ => Err(DiagnosticBuilder::default()
                .with_error_code(ParserError::InvalidExpr)
                .build()),
        }
    }
    fn parse_expr(&mut self, min_prec: u8, errors: &mut Vec<Diagnostic>) -> Option<Value> {
        let mut left = match self.parse_prefix() {
            Ok(value) => value,
            Err(err) => {
                errors.push(err);
                return None;
            }
        };
        loop {
            let kind = self.get_kind();
            if kind == TokenTag::EOF || kind == TokenTag::SemicolonOp {
                break;
            }
            let (op, prec) = match kind {
                TokenTag::PlusSign => (OpKind::Addition, 1),
                TokenTag::MinusSign => (OpKind::Subtraction, 1),
                TokenTag::StarSign => (OpKind::Multiplication, 2),
                TokenTag::DivSign => (OpKind::Division, 2),
                _ => {
                    return None;
                }
            };
            if prec < min_prec {
                break;
            }
            self.idx += 1; // consume op

            if let Some(right) = self.parse_expr(prec + 1, errors) {
                left = Value::BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    evaluated_ty: None,
                };
            } else {
                return None;
            }
        }
        Some(left)
    }
    fn parse_var(
        &mut self,
        errors: &mut Vec<Diagnostic>,
        desc: VariableDescriptor,
    ) -> Option<Statement> {
        self.idx += 1; // skip var

        let ident;
        {
            let token = self.get_token();
            if let Some(err) = self.expect(
                TokenTag::Identifier,
                TokenTag::SemicolonOp,
                ParserError::ExpectedIdentifier,
            ) {
                errors.push(err);
                return None;
            }
            ident = token.unwrap();
        }

        match self.expect_branch(
            &[TokenTag::EqualOp, TokenTag::SemicolonOp],
            &[TokenTag::EqualOp, TokenTag::SemicolonOp],
            ParserError::ExpectedEqual,
        ) {
            Ok(kind) => {
                if kind == TokenTag::SemicolonOp {
                    return Some(Statement::Variable(Variable::new(ident, None, None, desc)));
                }
            }
            Err((err, checkpoint)) => {
                errors.push(err);
                if checkpoint == TokenTag::EOF || checkpoint == TokenTag::SemicolonOp {
                    return None;
                } else {
                    // checkpoint == equal here
                    self.idx += 1;
                }
            }
        }
        let value = self.parse_expr(0, errors);
        if value.is_none() {
            self.skip_to(TokenTag::SemicolonOp);

            return None;
        }
        if let Some(err) = self.expect(
            TokenTag::SemicolonOp,
            TokenTag::SemicolonOp,
            ParserError::ExpectedSemicolon,
        ) {
            errors.push(err);
            return None;
        }
        Some(Statement::Variable(Variable::new(ident, None, value, desc)))
    }
}
