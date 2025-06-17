use std::{iter::Peekable, path::Iter, str::Chars, thread::panicking, vec::IntoIter};

use crate::reader::Reader;

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
    reader: Reader,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl Lexer {
    fn new(content: String) -> Self {
        Lexer {
            reader: Reader::new(content),
        }
    }

    fn next_token(&mut self) -> Token {
        let char = self.reader.peek();

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

        while let Some(c) = self.reader.next() {
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
        self.reader.next().unwrap();
        let mut content = String::new();
        let mut q_count = 0;
        let mut is_multitline = false;
        // let mut advanced = 0;

        // for to check if its a multiline string
        let mut i = 0;
        loop {
            if let Some(c) = self.reader.peek_nth(i) {
                if *c == '\'' {
                    q_count += 1;
                } else if *c == '\n' {
                    is_multitline = true;
                } else {
                    break;
                }
            } else {
                panic!("process_stringliteral: What")
            }

            i += 1;
        }

        self.reader.advance_by(q_count);

        // for content of the string
        // // for check if its the end of the string

        // self.it.advance_by(advanced);

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
