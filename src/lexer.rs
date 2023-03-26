use crate::definations::KNWON_SYMBOLS;
use std::str::Chars;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum TokenType {
    Number,
    Text,
    Symbol
}
#[derive(Debug)]
pub struct Token {
    pub token: String,
    token_type: TokenType,
    pub start_index: usize
}

impl Token {
    pub fn new(token: String, token_type: TokenType, start_index: usize) -> Token {
        Token{
            token,
            token_type,
            start_index
        }
    }
}

fn is_numeric(c:char)->bool {
    c as u8 >= 48 && c as u8 <=  57
}
fn is_alpha(c:char)->bool {
    (c as u8 >= 65 && c as u8 <=  90) || (c as u8 >= 97 && c as u8 <=  122)
}
#[derive(Debug)]
pub struct Lexer<'a> {
    index:usize,
    chars: Chars<'a>,
    buffer: Vec<char>,
    last_token_type: TokenType,
    optional_char: Option<char>
}
impl Lexer<'_>{
    pub fn new<'a>(expression: &'a str) ->Lexer<'a> {
        Lexer {
            chars: expression.chars(),
            index: 0,
            buffer: Vec::new(),
            last_token_type: TokenType::Text,
            optional_char: None
        }
    }
    fn next(&mut self) {
        self.optional_char = self.chars.next();
        if self.optional_char.is_some() {
            self.index = self.index + 1;
        }
    }
    
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if self.index == 0 { //Called first time
            self.next();
            if self.optional_char.is_none() {
                return None;
            }
            let c: char = self.optional_char.unwrap();
            if is_numeric(c) { //Number
                self.last_token_type = TokenType::Number;
                self.buffer.push(c)
            } else if KNWON_SYMBOLS.contains(&c) { // Known symbols
                self.last_token_type = TokenType::Symbol;
                self.buffer.push(c)
            } else { //Text
                self.last_token_type = TokenType::Text;
                self.buffer.push(c)
            }
        }
        self.next();
        let mut c: char;
        while self.optional_char.is_some() {
            c = self.optional_char.unwrap();
            if is_numeric(c) { //Number
                match self.last_token_type {
                    TokenType::Number => self.buffer.push(c),
                    TokenType::Text => {
                        self.last_token_type = TokenType::Text;
                        self.buffer.push(c)
                    },
                    TokenType::Symbol => {
                        if self.buffer[self.buffer.len()-1] == '.' {
                            self.buffer.push(c);    
                        } else {
                            let token = Token::new(self.buffer.iter().collect(), self.last_token_type, self.index-self.buffer.len() -1);
                            self.buffer.clear();
                            self.last_token_type = TokenType::Number;
                            self.buffer.push(c);
                            return Some(token)
                        }
                    },
                }
            } else if KNWON_SYMBOLS.contains(&c) { // Known symbols
                match self.last_token_type {
                    TokenType::Number => {
                        if c == '.' && !self.buffer.contains(&'.') {
                            self.buffer.push(c);    
                        } else {
                            let token = Token::new(self.buffer.iter().collect(), self.last_token_type, self.index-self.buffer.len() -1);
                            self.buffer.clear();
                            self.last_token_type = TokenType::Symbol;
                            self.buffer.push(c);
                            return Some(token)
                        }
                    },
                    TokenType::Text => {
                        let token = Token::new(self.buffer.iter().collect(), self.last_token_type, self.index-self.buffer.len() -1);
                        self.buffer.clear();
                        self.last_token_type = TokenType::Symbol;
                        self.buffer.push(c);
                        return Some(token)
                    },
                    TokenType::Symbol => {
                        let token = Token::new(self.buffer.iter().collect(), self.last_token_type, self.index-self.buffer.len() -1);
                        self.buffer.clear();
                        self.last_token_type = TokenType::Symbol;
                        self.buffer.push(c);
                        return Some(token)
                    },
                }
            } else { //Text
                match self.last_token_type {
                    TokenType::Number => {
                        self.last_token_type = TokenType::Text;
                        self.buffer.push(c);
                    },
                    TokenType::Text => {
                        self.buffer.push(c);
                    },
                    TokenType::Symbol => {
                        let token = Token::new(self.buffer.iter().collect(), self.last_token_type, self.index-self.buffer.len() -1);
                        self.buffer.clear();
                        self.last_token_type = TokenType::Text;
                        self.buffer.push(c);
                        return Some(token)
                    },
                }
            }
            self.next();
        }
        
        if self.buffer.is_empty() {
            return None;
        }
        let token = Token::new(self.buffer.iter().collect(), self.last_token_type, self.index-self.buffer.len() -1);
        self.buffer.clear();
        return Some(token)
    }
}