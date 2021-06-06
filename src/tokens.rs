use sqlparser::tokenizer::Tokenizer;
use sqlparser::dialect::GenericDialect;
use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::Word;
use sqlparser::tokenizer::Whitespace;

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
pub struct NormalizedTokens(pub Vec<Token>); // no whitespace, value removed

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
pub struct PrunedTokens(pub Vec<Token>); // no whitespace

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
pub struct RawTokens(pub Vec<Token>);

pub fn tokenize(req: &str) -> RawTokens {
    let dialect = GenericDialect {};
    RawTokens(Tokenizer::new(&dialect, req).tokenize().unwrap())
}

pub fn normalize_once(t: Token) -> Option<Token> {
    // the user may add whitespace but not comment !
    // remove the value of the tokens as well

    match t {
        Token::Word(w) => Some(Token::Word(Word { value: String::new(), quote_style: None, keyword: w.keyword })), // keep only the keyword
        Token::Number(_,_) => Some(Token::Number(String::from("0"),true)),
        Token::Char(_) => Some(Token::Char(' ')),
        Token::SingleQuotedString(_) => Some(Token::SingleQuotedString(String::new())),
        Token::NationalStringLiteral(_) => Some(Token::NationalStringLiteral(String::new())),
        Token::HexStringLiteral(_) => Some(Token::HexStringLiteral(String::new())),
        Token::Whitespace(Whitespace::Space) => None,
        Token::Whitespace(Whitespace::Newline) => None,
        Token::Whitespace(Whitespace::Tab) => None,
        _ => Some(t)
    }

}

pub fn is_whitespace(t: &Token) -> bool {
    match t {
        Token::Whitespace(Whitespace::Space) => true,
        Token::Whitespace(Whitespace::Newline) => true,
        Token::Whitespace(Whitespace::Tab) => true,
        _ => false
    }
}

pub fn prune(tok: RawTokens) -> PrunedTokens {
    PrunedTokens(tok.0.into_iter().filter(|t| !is_whitespace(&t)).collect())
}

pub fn normalize(tok: RawTokens) -> NormalizedTokens {
    NormalizedTokens(tok.0.into_iter().filter_map(|t| normalize_once(t)).collect())
}


