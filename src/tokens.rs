use sqlparser::tokenizer::Tokenizer;
use sqlparser::dialect::GenericDialect;
use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::Word;
use sqlparser::tokenizer::Whitespace;

    pub fn tokenize(req: &str) -> Vec<Token> {
        let dialect = GenericDialect {};
        Tokenizer::new(&dialect, req).tokenize().unwrap()
    }

    pub fn normalize_once(t: Token) -> Option<Token>
    {
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

    pub fn normalize(tok: &Vec<Token>) -> Vec<Token> {
        // the user may add whitespace but not comment !
        // remove the value of the tokens as well

        tok.clone().into_iter().filter_map(|t| normalize_once(t)).collect()
    }


