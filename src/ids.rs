use sqlparser::tokenizer::Tokenizer;
use sqlparser::dialect::GenericDialect;
use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::Whitespace;

use std::collections::HashMap;

use crate::template::Template;


#[derive(Debug)]
pub enum ReqFate {
    Pass(Option<Template>),
    Del
}

pub struct Ids {
    templates: Vec<Template>,
    lonely_queries: HashMap<usize, Vec<String>>
}

impl Ids {
    pub fn new() -> Ids {
        Ids { templates: Vec::new(), lonely_queries: HashMap::new() }
    }

    pub fn handle_req(&mut self, req: &str) -> ReqFate {
        let tokens = Ids::tokenize_without_whitespace(req);
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
            self.learn(req, tokens);
            ReqFate::Pass(None)
        }
    }

    fn learn(&mut self, req: &str, tokens: Vec<Token>) {
        let n = tokens.len();
        let queries = self.lonely_queries.entry(n).or_insert(Vec::new());
        queries.push(req.to_string());
        let u = 3;
        let a = "SELECT * FROM";
        let b = "WHERE A=1";
        let t = Template::new(&vec![a,b], vec![(tokens[u].clone(),u)]);
        self.templates.push(t);
        // println!("Legitimate: {}", t.is_legitimate(&tokens));

    }

    fn tokenize_without_whitespace(req: &str) -> Vec<Token> {
        let c = |t : &Token| {
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


}
