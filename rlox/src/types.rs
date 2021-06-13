use std::{result, error, io, fmt, str::fromStr};
use derive_more::{self, Error, From};
use strum_macros as strum;
pub type Result<T> = result::Result<T, Error>;
#[derive(Debug, Error)]
pub struct ParseError {
    kind : TokenKind,
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
pub struct Token {
    token_kind : TokenKind,
    //Store locations as offset + length, we need to count newlines
    //to provide specific locations, but it's worth it given that
    //errors are relatively rare.
    offset : usize,
    length : usize,
    value : Option<RloxValue>
}
impl for Token {
    pub fn new(kind : TokenKind, offset : usize,
               length : usize, value : Option<RloxValue>){
        Token { kind, offset, length, value }
    }
}
/*
 An enumeration representing the different kinds of tokens,
 each type may have further subdivisions.
 Would be called TokenType, but since type is a reserved word
 
*/
#[derive(Debug, strum::Display)]
pub(crate) enum TokenKind {
    //Tokens with an assoicated value
    Identifier,
    Float,
    Integer,
    String,
    //Tokens without an assoicated value
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation)
}
// impl From<&str> for TokenKind {
//     fn from(str : &str) -> Self {
//         Token::String(str.to_string())
//     }
// }
// macro_rules! impl_from_for_token {
//     $($from:ty) => {
//         impl From<$from> for TokenKind {
//             fn from(arg : $from){
//                 TokenKind::$from(arg)
//             }
//         }
//     }
// }

impl From<Operator> for TokenKind {
    fn from(op: Operator){
        TokenKind::Operator(op)
    }
}
impl From<Punctuation> for TokenKind {
    fn from(punct : Punctuation){
        TokenKind::Punctuation(punct)
    }
}
    
#[derive(Debug, strum::Display, strum::EnumString)]
pub(crate) enum Operator {
    //Arithmetic
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Sub,
    #[strum(serialize = "*")]
    Mul,
    #[strum(serialize = "/")]
    Div,
    //Comparison
    #[strum(serialize = "==")]
    Eq,
    #[strum(serialize = "!=")]
    NotEq,
    #[strum(serialize = "<=")]
    LtEq,
    #[strum(serialize = ">=")]
    GtEq,
    #[strum(serialize = "<")]
    Lt,
    #[strum(serialize = ">")]
    Gt,
    //Bitwise
    #[strum(serialize = "~")]
    BitNot,
    #[strum(serialize = "&")]
    BitAnd,
    #[strum(serialize = "|")]
    BitOr,
    #[strum(serialize = "^")]   
    BitXor,
    #[strum(serialize = "<<")]
    Shl, //Kept short to match the rest of the enum names
    #[strum(serialize = ">>")]
    Shr,
    //Logical
    #[strum(serialize = "!")]
    Not,
    #[strum(serialize = "&&")]
    And,
    #[strum(serialize = "||")]
    Or,
    //Assignment
    #[strum(serialize = "=")]
    Set,
    #[strum(serialize = "+=")]
    AddSet,
    #[strum(serialize = "-=")]
    SubSet,
    #[strum(serialize = "*=")]
    MulSet,
    #[strum(serialize = "/=")]
    DivSet,
    #[strum(serialize = "&=")]
    AndSet,
    #[strum(serialize = "|=")]
    OrSet,
    #[strum(serialize = "^=")]
    XorSet,
    #[strum(serialize = "<<=")]
    ShlSet,
    #[strum(serialize = ">>=")]
    ShrSet,
}
/*
  Not A Huge fan of having this be a seperate enum, but
  I'm not sure how to best seperate the different parts right now.
*/
#[derive(Debug, strum::Display, strum::EnumString)]
pub(crate) enum Punctuation {
    #[strum(serialize = "(")]
    LParen,
    #[strum(serialize = ")")]
    RParen,
    #[strum(serialize = "{")]
    LBrace,
    #[strum(serialize = "}")]
    RBrace,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = ".")]
    Period
}
#[derive(Debug, strum::Display, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Keyword {
    //Control Flow
    If, Else, For, While, Return, Break, Continue,
    //Data / Variables
    Var, Class, Fun, This, Super,
    //Literals
    True, False, Nil
}
pub enum RloxValue {
    Int(i64),
    Double(f64),
    String(String),
/*    Class(Class),
    Object(Object),
    Function(Function),
    Symbol(Symbol)*/
}
impl 
