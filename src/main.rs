mod parse_math;
use parse_math::parser::Parser;
use std::io;

fn main() {
    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Your input: {}", input);
                let mut parser = Parser::new(&input);
                match parser.evaluate() {
                    Ok(result) => println!("Result: {}", result),
                    Err(error) => println!("Parse error: {}", error),
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
