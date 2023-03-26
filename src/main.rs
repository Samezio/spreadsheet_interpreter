mod lexer;
pub mod definations;


use lexer::{Lexer, Token};
fn main() {
    let test_expression = "AVERAGE(125,485,987,45,0,-80,70) + 100 & USD";
    let mut lexer = Lexer::new(test_expression);
    let mut token = lexer.next_token();
    while token.is_some() {
        println!("{:?}", token);
        token = lexer.next_token();
    }
}
