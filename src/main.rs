use xync::{Lexer, Parser, Semantic};

fn main() {
    let mut lexer = Lexer::new("let a; {let a1; let b; let c = a + b - a1; let d = -a;}"); 
    let mut parser = Parser::new(match lexer.lex() {
        Ok(tokens) => {
            dbg!(tokens)
        }
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
            return;
        }
    });
    let scope_arena = match parser.parse() {
        Ok(scope_arena) => {
            dbg!(scope_arena)
        }
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
            return;
        }
    };
    let mut semantic = Semantic::new(scope_arena.size());
    match semantic.analyze(scope_arena) {
        Ok(()) => {
            dbg!(semantic);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    
}
