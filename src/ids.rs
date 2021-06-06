use sqlparser::tokenizer::Token;
//use std::sync::RwLock TODO

use std::collections::HashMap;

use crate::template::Template;
use crate::tokens::{tokenize,normalize};

const MINIMUM_QUERIES_FOR_LEARNING: usize = 5;

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
        let tokens = tokenize(req);
        let tokens_norm = normalize(&tokens);
        println!("{:?}",tokens);
        let u1 = 6;
        let u2 = 10;
        let t = Template::new(&tokens, vec![u1, u2]);

        println!("{:?}",tokens);
        println!("Req: {}",req);
        let mut match_found = false;
        for t in self.templates.iter() {
            if t.is_match(req) {
                println!("Match with {:?}", t);
                match_found = true;
                if t.is_legitimate(&tokens_norm) {
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
        if queries.len() >= MINIMUM_QUERIES_FOR_LEARNING {
            
        }
        // let t = Template::new(&vec![a,b], vec![(tokens[u].clone(),u)]);
        // self.templates.push(t);
        // println!("Legitimate: {}", t.is_legitimate(&tokens));

    }

}
