
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lex: Lexer,
}

impl Parser {
    pub fn new(content: String) -> Self {
        Parser { lex: Lexer::new(content) }
    }
}
pub struct File {
    name: String,
    interface: Vec<Interface>,
    implementation: String,
    initialization: String,
    finalization: String,
}

pub enum Interface {
    imports(Vec<String>),
    constant(Vec<(String, Token)>),
    definition,
    variable,
}