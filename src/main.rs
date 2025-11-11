mod lexer;
mod parser;
mod reader;

fn main() {
    // Test the lexer
    let mut lexer = lexer::Lexer::new(String::from("unit MyUnit; end."));
    let token1 = lexer.next();
    let token2 = lexer.next();
    let token3 = lexer.next();
    let token4 = lexer.next();
    let token5 = lexer.next();

    println!("Tokens: {:?}", [token1, token2, token3, token4, token5]);

    // Test the parser
    let content = String::from("unit MyUnit; end.");
    let mut parser = parser::Parser::new(content);

    match parser.parse_file() {
        Ok(()) => println!("Parsing successful!"),
        Err(e) => println!("Parsing failed: {}", e),
    }

    println!("Hello, Delphi linter!");
}
