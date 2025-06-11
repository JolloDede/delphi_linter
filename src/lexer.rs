use std::{iter::Peekable, path::Iter, str::Chars, vec::IntoIter};

#[derive(Debug, PartialEq)]
enum TokenTyp {
    Comment,
    String,
    Number,
    ConditionalCompilation,
    Operator,
    Keyword,
    Identifier,
    Whitespace,
    EOF,
}

struct Token {
    typ: TokenTyp,
    content: String,
    row: i64,
    col: i64,
}

struct Lexer {
    it: Peekable<IntoIter<char>>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl Lexer {
    fn new(content: String) -> Self {
        let charas: Vec<char> = content.chars().collect();
        Lexer {
            it: charas.into_iter().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        let char = self.it.peek();

        match char {
            Some(c) if c.is_whitespace() => self.process_whitespace(),
            Some(c) if *c == '\'' => self.process_stringliteral(),
            Some(_) => todo!(),
            None => Token {
                typ: TokenTyp::EOF,
                content: String::new(),
                row: 0,
                col: 0,
            },
        }
    }

    fn process_whitespace(&mut self) -> Token {
        let mut content = String::new();

        while let Some(c) = self.it.next() {
            if !c.is_whitespace() {
                break;
            }
            content.push(c);
        }

        return Token {
            typ: TokenTyp::Whitespace,
            content: content,
            row: 0,
            col: 0,
        };
    }

    fn process_stringliteral(&mut self) -> Token {
        self.it.next().unwrap();
        let mut content = String::new();
        let mut q_count = 0;

        if *self.it.peek().unwrap() == '\'' {
            while let Some(c) = self.it.next() {
                if c == '\'' {
                    q_count += 1;
                } else {
                    break;
                }
            }
        }

        while let Some(c) = self.it.next() {
            if c == '\'' {
                if q_count == 0 {
                    break;
                } else {
                    q_count -= 1;
                }
            } else {
                content.push(c);
            }
        }

        return Token {
            typ: TokenTyp::String,
            content: content,
            row: 0,
            col: 0,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, TokenTyp};

    #[test]
    fn eof_token() {
        let mut lex = Lexer::new(String::from(""));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::EOF);
    }

    #[test]
    fn whitespace_tokens() {
        let mut lex = Lexer::new(String::from(" \t\n"));

        let tok = lex.next_token();

        println!("{}", tok.content);

        assert_eq!(tok.typ, TokenTyp::Whitespace);
        assert_eq!(tok.content, " \t\n");
    }

    #[test]
    fn string_tokens() {
        let mut lex = Lexer::new(String::from("'string'"));

        let tok = lex.next_token();

        println!("{}", tok.content);

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "string");
    }

    #[test]
    fn multiline_string_tokens() {
        let mut lex = Lexer::new(String::from("'''\ncool\n'''"));

        let tok = lex.next_token();

        println!("{}", tok.content);

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "cool\n");
    }

    #[test]
    fn multiline_string_embeding_tokens() {
        let mut lex = Lexer::new(String::from("'''''\nco'''ol\n'''''"));

        let tok = lex.next_token();

        println!("{}", tok.content);

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "co'''ol");
    }

    #[test]
    fn string_escaped_tokens() {
        // let mut lex = Lexer::new(String::from(" \t\n"));

        // let tok = lex.next_token();

        // println!("{}", tok.content);

        // assert!(tok.typ == TokenTyp::Whitespace);
        // assert!(tok.content == " \t\n");
        todo!("bla")
    }
}
