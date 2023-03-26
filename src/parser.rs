use crate::lexer::{Lexer, Token, TokenType};
pub enum ASTType{
    BinaryOpertor,
    UnaryOperator,
    Function,
    Range,
    Cell,
    Text,
    Number
}
pub struct ATS {
    pub node: Token,
    pub ast_type: ASTType,
    pub childern: Vec<AST>
}

impl AST {
    pub fn new(node:Token, ast_type: ASTType) -> AST {
        AST{
            node,
            ast_type
        }
    }
}
pub struct Parser {
    expression: String,
    lexer: Lexer,
    current_token: Option<Token>
}
impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {lexer, token:None}
    }
    fn eat(&self, token_type: TokenType) -> Result<_,SyntaxError> {
        if self.current_token.token_type == token_type {
            self.current_token =  self.lexer.next();
            return Ok();
        }
        return Err(SyntaxError{
            token: self.current_token,
            expression: self.expression,
            message: Some("Unexpected token")
        });
    }
    pub fn build() -> Result<AST, SyntaxError> {
    }
}