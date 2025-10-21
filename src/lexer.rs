use core::panic;

use crate::reader::Reader;

#[derive(Debug, PartialEq)]
pub enum TokenTyp {
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

pub struct Token {
    pub typ: TokenTyp,
    pub content: String,
    pub row: usize,
    pub col: usize,
}

pub struct Lexer {
    reader: Reader,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Lexer {
            reader: Reader::new(content),
        }
    }

    pub fn next(&mut self) -> Token {
        self.next_token()
    }

    fn next_token(&mut self) -> Token {
        let char = self.reader.peek();

        match char {
            Some(c) if c.is_whitespace() => self.process_whitespace(),
            Some(c) if c == '\'' => self.process_stringliteral(),
            Some(c) if c.is_numeric() => self.process_numeric(),
            Some(c) if c == '{' || (c == '/' && self.reader.peek_nth(1).unwrap() == '/') => {
                self.process_comment()
            }
            Some(c) if ['+', '-', '*', '/'].contains(&c) => Token {
                typ: TokenTyp::Operator,
                content: self.reader.next().unwrap().to_string(),
                row: 0,
                col: 0,
            },
            Some(c) if c.is_alphabetic() => self.process_indentifier(),
            Some(_) => todo!(),
            None => Token {
                typ: TokenTyp::EOF,
                content: String::new(),
                row: self.reader.row,
                col: self.reader.col,
            },
        }
    }

    fn process_whitespace(&mut self) -> Token {
        let mut content = String::new();
        let row = self.reader.row;
        let col = self.reader.col;

        while let Some(c) = self.reader.next() {
            if !c.is_whitespace() {
                break;
            }
            content.push(c);
        }

        return Token {
            typ: TokenTyp::Whitespace,
            content: content,
            row: row,
            col: col,
        };
    }

    fn process_string(&mut self, quote_count: usize) -> Token {
        let mut content = String::new();
        let row = self.reader.row;
        let col = self.reader.col;

        // String only contains '
        if quote_count % 2 == 0 {
            if quote_count != 2 {
                content = self.delphi_string_escaped_quotes(quote_count);
            }
            return Token {
                typ: TokenTyp::String,
                content: content,
                row: row,
                col: col,
            };
        }

        loop {
            content += &self.reader.read_until('\'');

            let q_count = self.reader.count_until_not('\'');
            self.reader.advance_by(q_count);

            if q_count % 2 != 0 {
                break;
            }
        }

        return Token {
            typ: TokenTyp::String,
            content: content,
            row: row,
            col: col,
        };
    }

    fn delphi_string_escaped_quotes(&self, quote_count: usize) -> String {
        [1..(quote_count / 2) - 1].map(|_| '\'').iter().collect()
    }

    fn process_multiline_string(&mut self, quote_count: usize) -> Token {
        let mut content = String::new();
        let row = self.reader.row;
        let col = self.reader.col;

        self.reader.advance_by(quote_count);
        // Next must be \n
        self.reader.next();

        let indent_space_count = self.reader.count_until_not(' ');

        loop {
            self.reader.advance_by(indent_space_count);

            if self.reader.peek() == Some('\'') {
                let end_quote_count = self.reader.count_until_not('\'');

                if end_quote_count == quote_count {
                    break;
                } else {
                    content = self.delphi_string_escaped_quotes(end_quote_count);
                }
            }
            content += &self.reader.read_until_any(&['\n', '\'']);
        }

        // while let Some(c) = self.reader.next() {
        //     if c == '\'' {
        //         let current_q_count = self.string_count_quote();

        //         if is_multitline && current_q_count == q_count {
        //             self.reader.advance_by(current_q_count);
        //             break;
        //         } else if is_multitline {
        //             let _ = [0..current_q_count / 2].map(|_| content.push('\''));
        //         }
        //     } else {
        //         content.push(c);
        //     }
        // }

        content.truncate(content.len() - 1);

        return Token {
            typ: TokenTyp::String,
            content: content,
            row: row,
            col: col,
        };
    }

    fn process_stringliteral(&mut self) -> Token {
        let quote_count = self.reader.count_until_not('\'');

        if quote_count % 2 == 0 {
            return self.process_string(quote_count);
        } else {
            return self.process_multiline_string(quote_count);
        }

        // self.reader.next().unwrap();
        // let mut content = String::new();
        // let mut is_multitline = false;

        // let row = self.reader.row;
        // let col = self.reader.col;

        // let q_count = self.string_count_quote();

        // if q_count > 1 {
        //     self.reader.advance_by(q_count);

        //     if let Some(c) = self.reader.peek() {
        //         if c == '\n' {
        //             is_multitline = true;
        //         } else {
        //             let _ = [0..q_count / 2].map(|_| content.push('\''));
        //         }
        //     }
        // }

        // while let Some(c) = self.reader.next() {
        //     if c == '\'' {
        //         let current_q_count = self.string_count_quote();

        //         if is_multitline && current_q_count == q_count {
        //             self.reader.advance_by(current_q_count);
        //             break;
        //         } else if is_multitline {
        //             let _ = [0..current_q_count / 2].map(|_| content.push('\''));
        //         }
        //     } else {
        //         content.push(c);
        //     }
        // }

    }

    fn process_numeric(&mut self) -> Token {
        let mut content = String::new();

        let row = self.reader.row;
        let col = self.reader.col;

        while let Some(c) = self.reader.next() {
            if c.is_numeric() || c == '.' {
                content.push(c);
            } else {
                break;
            }
        }

        return Token {
            typ: TokenTyp::Number,
            content: content,
            row: row,
            col: col,
        };
    }

    fn process_indentifier(&mut self) -> Token {
        const DELPHI_KEYWORDS: [&str; 105] = [
            "absolute",
            "abstract",
            "and",
            "array",
            "as",
            "asm",
            "assembler",
            "automated",
            "begin",
            "case",
            "cdecl",
            "class",
            "const",
            "constructor",
            "contains",
            "default",
            "destructor",
            "dispid",
            "dispinterface",
            "div",
            "do",
            "downto",
            "dynamic",
            "else",
            "end",
            "except",
            "export",
            "exports",
            "external",
            "far",
            "file",
            "final",
            "finalization",
            "finally",
            "for",
            "forward",
            "function",
            "goto",
            "if",
            "implementation",
            "implements",
            "in",
            "index",
            "inherited",
            "initialization",
            "inline",
            "interface",
            "is",
            "label",
            "library",
            "message",
            "mod",
            "name",
            "near",
            "nil",
            "not",
            "object",
            "of",
            "on",
            "or",
            "out",
            "overload",
            "override",
            "package",
            "packed",
            "pascal",
            "platform",
            "private",
            "procedure",
            "program",
            "property",
            "protected",
            "public",
            "published",
            "raise",
            "read",
            "record",
            "register",
            "reintroduce",
            "repeat",
            "requires",
            "resident",
            "resourcestring",
            "safecall",
            "set",
            "shl",
            "shr",
            "stdcall",
            "stored",
            "string",
            "then",
            "threadvar",
            "to",
            "try",
            "type",
            "unit",
            "unsafe",
            "until",
            "uses",
            "var",
            "virtual",
            "while",
            "with",
            "write",
            "xor",
        ];
        let mut content = String::new();

        let row = self.reader.row;
        let col = self.reader.col;

        if let Some(c) = self.reader.next() {
            if c.is_alphabetic() {
                content.push(c);
            } else {
                panic!("process_indentifier is_alphabetic");
            }
        } else {
            panic!("process_indentifier next");
        }

        while let Some(c) = self.reader.next() {
            if c.is_alphanumeric() {
                content.push(c);
            } else {
                break;
            }
        }

        let typ = if DELPHI_KEYWORDS.contains(&content.as_str()) {
            TokenTyp::Keyword
        } else {
            TokenTyp::Identifier
        };

        return Token {
            typ: typ,
            content: content,
            row: row,
            col: col,
        };
    }

    fn process_comment(&mut self) -> Token {
        let mut content = String::new();
        let start_char = self.reader.peek().unwrap();

        let row = self.reader.row;
        let col = self.reader.col;

        match start_char {
            '/' => {
                self.reader.advance_by(2);
                while let Some(c) = self.reader.next() {
                    if c != '\n' {
                        content.push(c);
                    } else {
                        break;
                    }
                }
            }
            '{' => {
                self.reader.advance_by(1);
                while let Some(c) = self.reader.next() {
                    if c != '}' {
                        content.push(c);
                    } else {
                        break;
                    }
                }
            }
            _ => panic!("WTF how did we get here"),
        }

        return Token {
            typ: TokenTyp::Comment,
            content: content,
            row: row,
            col: col,
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

        assert_eq!(tok.typ, TokenTyp::Whitespace);
        assert_eq!(tok.content, " \t\n");
    }

    #[test]
    fn string_tokens() {
        let mut lex = Lexer::new(String::from("'string'"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "string");
    }

    #[test]
    fn empty_string() {
        let mut lex = Lexer::new(String::from("''"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "");

        let mut lex = Lexer::new(String::from("''''"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "'");
    }

    #[test]
    fn multiline_string_tokens() {
        let mut lex = Lexer::new(String::from("'''\ncool\n'''"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "cool");
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

    #[test]
    fn numeric_tokens() {
        let mut lex = Lexer::new(String::from("12"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Number);
        assert_eq!(tok.content, "12");

        let mut lex = Lexer::new(String::from("1.2"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Number);
        assert_eq!(tok.content, "1.2");
    }

    #[test]
    fn operator_tokens() {
        let mut lex = Lexer::new(String::from("*"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Operator);
        assert_eq!(tok.content, "*");

        let mut lex = Lexer::new(String::from(":"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Operator);
        assert_eq!(tok.content, ":");

        let mut lex = Lexer::new(String::from("="));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Operator);
        assert_eq!(tok.content, "=");
    }

    #[test]
    fn indentifier_tokens() {
        let mut lex = Lexer::new(String::from("variable"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Identifier);
        assert_eq!(tok.content, "variable");
    }

    #[test]
    fn keyword_tokens() {
        let mut lex = Lexer::new(String::from("const"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Keyword);
        assert_eq!(tok.content, "const");
    }

    #[test]
    fn comment_tokens() {
        let mut lex = Lexer::new(String::from("// nice comment\n"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Comment);
        assert_eq!(tok.content, " nice comment");

        let mut lex = Lexer::new(String::from("{another one}"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Comment);
        assert_eq!(tok.content, "another one");
    }
}
