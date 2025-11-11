use crate::lexer::{Lexer, Token, TokenTyp};

pub struct Parser {
    lex: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(content: String) -> Self {
        let mut parser = Parser {
            lex: Lexer::new(content),
            current_token: None,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = Some(self.lex.next());
    }

    fn peek(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    fn match_token(&self, expected_type: TokenTyp) -> bool {
        if let Some(token) = &self.current_token {
            token.typ == expected_type
        } else {
            false
        }
    }

    fn consume(&mut self, expected_type: TokenTyp) -> Option<Token> {
        if self.match_token(expected_type) {
            let result = self.current_token.clone().unwrap();
            self.advance();
            Some(result)
        } else {
            None
        }
    }

    fn expect(&mut self, expected_type: TokenTyp) -> Token {
        if let Some(token) = self.consume(expected_type) {
            token
        } else {
            panic!(
                "Expected {:?} but found {:?}",
                expected_type,
                self.current_token.as_ref().map(|t| &t.typ)
            );
        }
    }

    pub fn parse_file(&mut self) -> Result<(), String> {
        // Parse file header: unit Identifier ;
        self.expect(TokenTyp::Keyword); // "unit"
        self.expect(TokenTyp::Identifier); // Identifier

        // Skip whitespace and expect semicolon
        loop {
            if self.match_token(TokenTyp::Operator) {
                if let Some(token) = self.peek() {
                    if token.content == ";" {
                        self.advance();
                        break;
                    }
                }
            }
            self.advance();
        }

        // Parse end.
        self.expect(TokenTyp::Operator); // "end."

        Ok(())
    }
}
