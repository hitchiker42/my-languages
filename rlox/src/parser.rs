use crate::types::*;
pub(crate) fn scan(source : &str) -> Result<Vec<Token>> {
    source.split(" \t\n").map(|x| Ok(Token::from(x))).collect()
}
