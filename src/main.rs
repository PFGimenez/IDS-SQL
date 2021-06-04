use sqlparser::tokenizer::Tokenizer;
use sqlparser::dialect::GenericDialect;

use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::Whitespace;
mod template;
use template::Template;

fn main() {
    let dialect = GenericDialect {};
    let req = "SELECT * FROM taable WHERE A=1 -- truc";
    let tokens = Tokenizer::new(&dialect, req).tokenize().unwrap();
    let tokens = tokenize_without_whitespace(req);

    println!("{:?}", tokens);
    println!("{:?}", tokens[0]);

    let mut tok : Vec<(Token,bool)> = tokens.into_iter().map(|t| (t,false)).collect();
    tok[6].1 = true;
    let a = "SELECT * FROM";
    let b = "WHERE A=1";
    let t = Template::new(&vec![a,b], tok);

    println!("{} {}", t.is_match("TEST"), t.is_match(req));
    let req2 = "SELECT * FROM table2 WHERE A=1";
    let tokens = Tokenizer::new(&dialect, req2).tokenize().unwrap();
    println!("{}", t.is_legitimate(&tokens));
}

fn tokenize_without_whitespace(req: &str) -> Vec<Token>
{
    let c = |t : &Token|
    {
        // the user may add whitespace but not comment !
        match t {
            Token::Whitespace(Whitespace::Space) => false,
            Token::Whitespace(Whitespace::Newline) => false,
            Token::Whitespace(Whitespace::Tab) => false,
            _ => true
        }
    };

    let dialect = GenericDialect {};
    Tokenizer::new(&dialect, req).tokenize().unwrap().into_iter().filter(c).collect()
}


