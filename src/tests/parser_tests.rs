/*
    Head: Parser tests module
    Description: Stress and capability testing for Parser

    @author LightMayo
*/

#[cfg(test)]
mod parser_tests {

    use crate::{
        DiagnosticBuilder, Lexer, Mutability, OpKind, Parser, ParserError, ScopeType, Statement,
        Token, Value, Variable, VariableDescriptor,
    };

    #[test]
    pub fn file_scope_initiated() {
        let mut parser = Parser::new(Lexer::new("let a = 12;").lex().unwrap());
        let scope_arena = parser.parse().ok().unwrap();
        let file_scope = scope_arena.get_file_scope().unwrap();
        // ? File scope don't have a parent
        assert_eq!(file_scope.get_parent(), None);
        // ? A file scope also with it's inner scopes will have a type Local
        assert_eq!(file_scope.get_scope_type(), ScopeType::Local)
    }

    #[test]
    pub fn invalid_statement() {
        let mut parser = Parser::new(Lexer::new("idk a = 12;").lex().unwrap());
        assert_eq!(
            parser.parse().err().unwrap(),
            vec![
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::InvalidStartOfStatement)
                    .build()
            ]
        )
    }

    #[test]
    pub fn missing_equal() {
        let mut parser = Parser::new(Lexer::new("let a 12;").lex().unwrap());
        assert_eq!(
            parser.parse().err().unwrap(),
            vec![
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::ExpectedEqual)
                    .build()
            ]
        )
    }

    #[test]
    pub fn missing_identifier() {
        let mut parser = Parser::new(Lexer::new("let = 12;").lex().unwrap());
        assert_eq!(
            parser.parse().err().unwrap(),
            vec![
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::ExpectedIdentifier)
                    .build()
            ]
        )
    }

    #[test]
    pub fn missing_semicolon() {
        let mut parser = Parser::new(Lexer::new("let a = 12").lex().unwrap());
        assert_eq!(
            parser.parse().err().unwrap(),
            vec![
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::ExpectedSemicolon)
                    .build()
            ]
        )
    }

    #[test]
    pub fn missing_expr() {
        let mut parser = Parser::new(Lexer::new("let a = ;").lex().unwrap());
        assert_eq!(
            parser.parse().err().unwrap(),
            vec![
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::ExpectedExpr)
                    .build()
            ]
        )
    }

    #[test]
    pub fn invalid_expr() {
        let mut parser = Parser::new(Lexer::new("let a = /;").lex().unwrap());
        assert_eq!(
            parser.parse().err().unwrap(),
            vec![
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::InvalidExpr)
                    .build()
            ]
        )
    }

    #[test]
    pub fn unfinished_prefix() {
        let mut parser = Parser::new(Lexer::new("let a = -;let b = +;").lex().unwrap());
        assert_eq!(
            parser.parse().err().unwrap(),
            vec![
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::ExpectedExpr)
                    .build(),
                DiagnosticBuilder::default()
                    .with_error_code(ParserError::ExpectedExpr)
                    .build()
            ]
        )
    }

    // ----------------------------------------------------------------------------------------

    #[test]
    pub fn no_value() {
        let mut parser = Parser::new(Lexer::new("let a;").lex().unwrap());
        assert_eq!(
            parser
                .parse()
                .ok()
                .unwrap()
                .get_file_scope()
                .unwrap()
                .get_statements(),
            &vec![Statement::Variable(Variable::new(
                Token::new("a"),
                None,
                None,
                VariableDescriptor::default(Mutability::Immutable)
            ))]
        )
    }

    #[test]
    pub fn with_int_value() {
        let mut parser = Parser::new(Lexer::new("let a = 13;").lex().unwrap());
        assert_eq!(
            parser
                .parse()
                .ok()
                .unwrap()
                .get_file_scope()
                .unwrap()
                .get_statements(),
            &vec![Statement::Variable(Variable::new(
                Token::new("a"),
                None,
                Some(Value::Integer("13".into())),
                VariableDescriptor::default(Mutability::Immutable)
            ))]
        )
    }

    #[test]
    pub fn operator_and_precedence() {
        let mut parser = Parser::new(Lexer::new("let a = -5 + 10 * +3;").lex().unwrap());
        assert_eq!(
            parser
                .parse()
                .ok()
                .unwrap()
                .get_file_scope()
                .unwrap()
                .get_statements(),
            &vec![Statement::Variable(Variable::new(
                Token::new("a"),
                None,
                Some(Value::BinaryExpr {
                    left: Box::new(Value::Negate(Box::new(Value::Integer("5".into())))),
                    op: OpKind::Addition,
                    right: Box::new(Value::BinaryExpr {
                        left: Box::new(Value::Integer("10".into())),
                        op: OpKind::Multiplication,
                        right: Box::new(Value::Absolute(Box::new(Value::Integer("3".into())))),
                        evaluated_ty: None
                    }),
                    evaluated_ty: None
                }),
                VariableDescriptor::default(Mutability::Immutable)
            ))]
        )
    }

    #[test]
    pub fn with_absolute_int_value() {
        let mut parser = Parser::new(
            Lexer::new("let a = +13;let b = +-22;let c = +++33;")
                .lex()
                .unwrap(),
        );
        assert_eq!(
            parser
                .parse()
                .ok()
                .unwrap()
                .get_file_scope()
                .unwrap()
                .get_statements(),
            &vec![
                Statement::Variable(Variable::new(
                    Token::new("a"),
                    None,
                    Some(Value::Absolute(Box::new(Value::Integer("13".into())))),
                    VariableDescriptor::default(Mutability::Immutable)
                )),
                Statement::Variable(Variable::new(
                    Token::new("b"),
                    None,
                    Some(Value::Absolute(Box::new(Value::Integer("22".into())))), // absolute to -x will give abs(x)
                    VariableDescriptor::default(Mutability::Immutable)
                )),
                Statement::Variable(Variable::new(
                    Token::new("c"),
                    None,
                    Some(Value::Absolute(Box::new(Value::Integer("33".into())))), // recursive absolute for x is just abs(x)
                    VariableDescriptor::default(Mutability::Immutable)
                )),
            ]
        )
    }

    #[test]
    pub fn with_negate_int_value() {
        let mut parser = Parser::new(
            Lexer::new("let a = -13;let b = -+22;let c = --2; let d = ---3;")
                .lex()
                .unwrap(),
        );
        assert_eq!(
            parser
                .parse()
                .ok()
                .unwrap()
                .get_file_scope()
                .unwrap()
                .get_statements(),
            &vec![
                Statement::Variable(Variable::new(
                    Token::new("a"),
                    None,
                    Some(Value::Negate(Box::new(Value::Integer("13".into())))),
                    VariableDescriptor::default(Mutability::Immutable)
                )),
                Statement::Variable(Variable::new(
                    Token::new("b"),
                    None,
                    Some(Value::Negate(Box::new(Value::Absolute(Box::new(
                        Value::Integer("22".into())
                    ))))),
                    VariableDescriptor::default(Mutability::Immutable)
                )),
                Statement::Variable(Variable::new(
                    Token::new("c"),
                    None,
                    Some(Value::Integer("2".into())), // even number of negations for x will give x
                    VariableDescriptor::default(Mutability::Immutable)
                )),
                Statement::Variable(Variable::new(
                    Token::new("d"),
                    None,
                    Some(Value::Negate(Box::new(Value::Integer("3".into())))), // odd number of negations for x will give -x
                    VariableDescriptor::default(Mutability::Immutable)
                ))
            ]
        )
    }

    #[test]
    pub fn nested_scopes() {
        let mut parser = Parser::new(
            Lexer::new("let a = 1; { let b = 2; { let c = 3; }}")
                .lex()
                .unwrap(),
        );

        // ! FILE_SCOPE / SCOPE A

        let scope_arena = parser.parse().ok().unwrap();
        let file_scope = scope_arena.get_file_scope().unwrap();
        // ? The file scope variables
        assert_eq!(
            file_scope.get_statements(),
            &vec![Statement::Variable(Variable::new(
                Token::new("a"),
                None,
                Some(Value::Integer("1".into())),
                VariableDescriptor::default(Mutability::Immutable)
            ))]
        );
        // ? The file scope children scopes
        assert_eq!(file_scope.get_children_scopes().len(), 1);

        // ! SCOPE B

        let scope_b_id = *file_scope.get_children_scopes().first().unwrap();
        // ? The file scope id is 0, so the next scope id which is scope `b` must be 1
        assert_eq!(scope_b_id, 1);
        let scope_b = scope_arena.get_scope(scope_b_id).unwrap();
        assert_eq!(
            scope_b.get_statements(),
            &vec![Statement::Variable(Variable::new(
                Token::new("b"),
                None,
                Some(Value::Integer("2".into())),
                VariableDescriptor::default(Mutability::Immutable)
            ))]
        );
        assert_eq!(scope_b.get_children_scopes().len(), 1);

        // ! SCOPE C

        let scope_c_id = *scope_b.get_children_scopes().first().unwrap();
        assert_eq!(scope_c_id, 2);
        let scope_c = scope_arena.get_scope(scope_c_id).unwrap();
        assert_eq!(
            scope_c.get_statements(),
            &vec![Statement::Variable(Variable::new(
                Token::new("c"),
                None,
                Some(Value::Integer("3".into())),
                VariableDescriptor::default(Mutability::Immutable)
            ))]
        );
        assert_eq!(scope_c.get_children_scopes().is_empty(), true);
    }

    #[test]
    pub fn deeper_nested_scopes() {
        let mut parser = Parser::new(Lexer::new("{ { { let d = 4; } } }").lex().unwrap());

        // ! FILE_SCOPE / SCOPE 0

        let scope_arena = parser.parse().ok().unwrap();
        let file_scope = scope_arena.get_file_scope().unwrap();
        // ? The file scope variables
        assert_eq!(file_scope.get_statements().is_empty(), true);
        // ? The file scope children scopes
        assert_eq!(file_scope.get_children_scopes().len(), 1);

        // ! SCOPE 1

        let scope_1_id = *file_scope.get_children_scopes().first().unwrap();
        // ? The file scope id is 0, so the next scope id which is scope `b` must be 1
        assert_eq!(scope_1_id, 1);
        let scope_1 = scope_arena.get_scope(scope_1_id).unwrap();
        assert_eq!(scope_1.get_statements().is_empty(), true);
        assert_eq!(scope_1.get_children_scopes().len(), 1);

        // ! SCOPE 2

        let scope_2_id = *scope_1.get_children_scopes().first().unwrap();
        assert_eq!(scope_2_id, 2);
        let scope_2 = scope_arena.get_scope(scope_2_id).unwrap();
        assert_eq!(scope_2.get_statements().is_empty(), true);
        assert_eq!(scope_2.get_children_scopes().len(), 1);

        // ! SCOPE 3

        let scope_3_id = *scope_2.get_children_scopes().first().unwrap();
        assert_eq!(scope_3_id, 3);
        let scope_3 = scope_arena.get_scope(scope_3_id).unwrap();
        assert_eq!(
            scope_3.get_statements(),
            &vec![Statement::Variable(Variable::new(
                Token::new("d"),
                None,
                Some(Value::Integer("4".into())),
                VariableDescriptor::default(Mutability::Immutable)
            ))]
        );
        assert_eq!(scope_3.get_children_scopes().is_empty(), true);
    }
}
