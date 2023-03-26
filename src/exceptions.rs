use crate::lexer::{Token};
use std::{error::Error, fmt};
use colored::*;


#[derive(Debug)]
struct SyntaxError{
    token:Token,
    expression: String,
    message: Option<String>
}

impl Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.message.is_some() {
            return write!(f, "{} after{} : {}", "Syntax Error".red().bold(), self.expression.chars().skip(self.token.start_index).take(self.token.token.len()).collect::<String>().yellow().bold(), self.message.as_ref().unwrap());
        }
        return write!(f, "{} after{}", "Syntax Error".red().bold(), self.expression.chars().skip(self.token.start_index).take(self.token.token.len()).collect::<String>().yellow().bold());
    }
}