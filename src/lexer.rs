use crate::reader::Reader;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone)]
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

        Token {
            typ: TokenTyp::Whitespace,
            content,
            row,
            col,
        }
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
                content,
                row,
                col,
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

        Token {
            typ: TokenTyp::String,
            content,
            row,
            col,
        }
    }

    fn delphi_string_escaped_quotes(&self, quote_count: usize) -> String {
        let mut result = String::new();
        for _ in 1..(quote_count / 2) - 1 {
            result.push('\'');
        }
        result
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

        content.truncate(content.len() - 1);

        Token {
            typ: TokenTyp::String,
            content,
            row,
            col,
        }
    }

    fn process_stringliteral(&mut self) -> Token {
        let quote_count = self.reader.count_until_not('\'');

        if quote_count % 2 == 0 {
            self.process_string(quote_count)
        } else {
            self.process_multiline_string(quote_count)
        }
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

        Token {
            typ: TokenTyp::Number,
            content,
            row,
            col,
        }
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
            if c.is_alphanumeric() || c == '_' {
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

        Token {
            typ,
            content,
            row,
            col,
        }
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

        Token {
            typ: TokenTyp::Comment,
            content,
            row,
            col,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, TokenTyp};

    #[test]
    fn test_eof_token() {
        let mut lex = Lexer::new(String::from(""));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::EOF);
    }

    #[test]
    fn test_whitespace_tokens() {
        let mut lex = Lexer::new(String::from(" \t\n"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Whitespace);
        assert_eq!(tok.content, " \t\n");
    }

    #[test]
    fn test_string_tokens() {
        let mut lex = Lexer::new(String::from("'string'"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "string");
    }

    #[test]
    fn test_empty_string_tokens() {
        let mut lex = Lexer::new(String::from("''"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::String);
        assert_eq!(tok.content, "");
    }

    #[test]
    fn test_numeric_tokens() {
        let mut lex = Lexer::new(String::from("123"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Number);
        assert_eq!(tok.content, "123");
    }

    #[test]
    fn test_operator_tokens() {
        let mut lex = Lexer::new(String::from("*"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Operator);
        assert_eq!(tok.content, "*");
    }

    #[test]
    fn test_identifier_tokens() {
        let mut lex = Lexer::new(String::from("variable"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Identifier);
        assert_eq!(tok.content, "variable");
    }

    #[test]
    fn test_keyword_tokens() {
        let mut lex = Lexer::new(String::from("const"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Keyword);
        assert_eq!(tok.content, "const");
    }

    #[test]
    fn test_comment_tokens() {
        let mut lex = Lexer::new(String::from("// nice comment\n"));

        let tok = lex.next_token();

        assert_eq!(tok.typ, TokenTyp::Comment);
        assert_eq!(tok.content, " nice comment");
    }
}
