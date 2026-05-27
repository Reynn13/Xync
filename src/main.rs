use xync::frontend::{Interner, lexer::Lexer};

fn main() {
    let mut lexer = Lexer::new("let a = 12; // a".as_bytes());
    for token in lexer.lex(&mut Interner::with_predefined_keywords()) {
        println!("{token}");
    }
}
