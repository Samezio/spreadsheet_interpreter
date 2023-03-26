pub mod lexer;
pub mod exceptions;
pub mod definations;
pub mod parser;


use lexer::{Lexer, Token};
fn main() {
    let test_expression = "AVERAGE(125,485,987,45,0,-80,70) + 100 & USD";
    let lexer = Lexer::new(test_expression);
    for token in lexer {
        println!("{:?}", token);
    }
}
