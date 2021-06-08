use std::{result, error, io, fmt};
use derive_more::{self, Error, From};
use strum_macros as strum;
pub type Result<T> = result::Result<T, Error>;
#[derive(Debug, Error)]
pub struct ParseError {
    line : usize,
    column : usize,
    msg : String
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}.{}] Error : {}",
               self.line, self.column, self.msg)
    }
}
#[derive(Debug, derive_more::Display, Error, From)]
pub enum Error {
    Io(io::Error),
    Parse(ParseError)
}
#[derive(Debug, strum::Display)]
pub(crate) enum TokenType {    
    Identifier(String),
    Number(f64),
    String(String),
    Keyword(Keyword),
    Op(Operator)
}
impl From<&str> for Token {
    fn from(str : &str) -> Self {
        Token::String(str.to_string())
    }
}        
#[derive(Debug, strum::Display, strum::EnumString)]
pub(crate) enum Operator {
    //Arithmetic
    Add, Sub, Mul, Div,
    //Comparison
    Eq, NotEq, LtEq, GtEq, Lt, Gt,
    //Bitwise
    BitNot, BitAnd, BitOr, BitXor,
    //Logical
    Not, And, Or
}
/*
  Not A Huge fan of having this be a seperate enum, but 
  I'm not sure how to best seperate the different parts right now. 
*/
#[derive(Debug, strum::Display, strum::EnumString)]
pub(crate) enum Punctuation {
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semicolon
}
    
    
#[derive(Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Keyword {
    //Control Flow
    If, Else, For, While, Return, Break, Continue,
    //Data / Variables
    Var, Class, Fun, This, Super,
    //Literals
    True, False, Nil,
    
pub enum LoxValue {
}   
        
