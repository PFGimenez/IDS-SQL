use sqlparser::tokenizer::Tokenizer;
use sqlparser::dialect::GenericDialect;
use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::Word;
use sqlparser::tokenizer::Whitespace;

//use std::sync::RwLock TODO

use std::collections::HashMap;

use crate::template::Template;

const MINIMUM_QUERIES_FOR_LEARNING: u32 = 5;

#[derive(Debug)]
pub enum ReqFate {
    Pass(Option<Template>),
    Del
}

pub struct Ids {
    templates: Vec<Template>,
    lonely_queries: HashMap<Vec<Token>, Vec<String>>
}

impl Ids {
    pub fn new() -> Ids {
        Ids { templates: Vec::new(), lonely_queries: HashMap::new() }
    }

    pub fn handle_req(&mut self, req: &str) -> ReqFate {
        let tokens = Ids::tokenize(req);
        println!("{:?}",tokens);
        let mut match_found = false;
        for t in self.templates.iter() {
            if t.is_match(req)
            {
                println!("Match with {:?}", t);
                match_found = true;
                if t.is_legitimate(&tokens)
                {
                    return ReqFate::Pass(Some(t.clone()));
                }
            }
        }
        if match_found {
            ReqFate::Del
        } else {
            self.learn(req, tokens); // TODO dans un thread ?
            ReqFate::Pass(None)
        }
    }

    fn learn(&mut self, req: &str, tokens: Vec<Token>) {
        let queries = self.lonely_queries.entry(tokens).or_insert(Vec::new());
        queries.push(req.to_string());
        if queries.len() >= MINIMUM_QUERIES_FOR_LEARNING
        {

        }
        // let t = Template::new(&vec![a,b], vec![(tokens[u].clone(),u)]);
        // self.templates.push(t);
        // println!("Legitimate: {}", t.is_legitimate(&tokens));

    }

    fn tokenize(req: &str) -> Vec<Token> {
        // the user may add whitespace but not comment !
        // remove the value of the tokens as well
        let filter_map = |t : Token| {
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
        };

        let dialect = GenericDialect {};
        Tokenizer::new(&dialect, req).tokenize().unwrap().into_iter().filter_map(filter_map).collect()
    }


}
