use crate::types::*;
const number_re = r"([0-9]+(?:.[0-9]*))";
const operator_re = r"([-+*/!~]|(:?[=<>]=?)|(?:&&?)|(?:\|\|?))";
const punctuation_re = r"([{}(),;])";
const keyword_or_ident_re = r"(\w+)";
const seperator = b"()!+/*<>=^-,; \n\t";
pub(crate) fn scan(source : &str) -> Result<Vec<Token>> {
    let tokens = Vec::new();
    let idx : usize = 0;
    let line : usize = 0;
    /*
       Obviously our source text is utf8, but since the only non ascii
       text will be identifiers it is much eaiser to treat our input
       as bytes and interpret it as utf8 only where necessary.
     */
    let sbytes = source.as_bytes();
    while(idx <= sbytes.len()){
        let start = idx;
        let token = match sbytes[idx] {
            //Comments/Whitespace
            b' '|'\n'|'\t' => {
                idx += 1;
                while sbytes[idx].is_ascii_whitespace() && idx < sbytes.len() {
                    idx += 1;
                }
                continue;
            }
            b'/' => {
                if 
            //One character tokens
            b'('|b')'|b'{'|b'}'|b';'|b','|b'<'|b'>'|
            b'~'|b'+'|b'-'|b'/'|b'*'|b'^'|b'='|b'!' => {
                idx += 1;
                Token::new(&source[idx-1..idx],
                           start, 1, None)
            }
            b'.' => {
                if sbytes[idx+1].is_ascii_digit() {
                    //make a number
                } else {
                    idx += 1;
                    Token::new(Punctuation::Period, start, 1, None)
                }
            }
            b'0' => {
                if sbytes[idx+1] == b'x' {
                    idx += 2;
                    while sbytes[idx].is_ascii_hexdigit() {
                        idx += 1;
                    }
                    let val = u64::from_str_radix(&source[start+2..idx], 16)?;
                    Token::new(TokenKind::Integer, start, idx, Some(val))
                }
            }
            b'1'|b'2'|b'3'|b'4'|b'5'|b'6'|b'7'|b'8'|b'9'|

}
fn match_keyword_or_ident(source : &str) -> 
