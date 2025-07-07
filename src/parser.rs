
use crate::lexer::Lexer;

pub struct Parser {
    lex: Lexer,
}

impl Parser {
    pub fn new(content: String) -> Self {
        Parser { lex: Lexer::new(content) }
    }
}