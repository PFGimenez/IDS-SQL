//use std::sync::RwLock TODO

use std::collections::HashMap;

use crate::template::Template;
use crate::tokens::{tokenize,normalize,prune,RawTokens,NormalizedTokens, PrunedTokens};

const MINIMUM_QUERIES_FOR_LEARNING: usize = 3;

#[derive(Debug)]
pub enum ReqFate {
    Pass(Option<Template>),
    Del
}

pub struct Ids {
    templates: Vec<Template>,
    unmatched_queries: HashMap<NormalizedTokens, Vec<PrunedTokens>> // index: normalized. value. not normalized
}

impl Ids {
    pub fn new() -> Ids {
        Ids { templates: Vec::new(), unmatched_queries: HashMap::new() }
    }

    pub fn handle_req(&mut self, req: &str) -> ReqFate {
        let tokens = tokenize(req);
        let norm_tokens = normalize(tokens.clone());
        println!("Req: {}",req);
        let mut match_found = false;
        for t in self.templates.iter() {
            if t.is_match(req) {
                println!("Match with {:?}", t);
                match_found = true;
                if t.is_legitimate(&norm_tokens) {
                    return ReqFate::Pass(Some(t.clone()));
                }
            }
        }
        if match_found {
            ReqFate::Del
        } else {
            self.learn(tokens); // TODO dans un thread ?
            ReqFate::Pass(None)
        }
    }


    fn learn(&mut self, tokens: RawTokens) {
        let pruned_tokens = prune(tokens.clone());
        let norm_tokens = normalize(tokens.clone());
        println!("{:?}", pruned_tokens);
        let queries = self.unmatched_queries.entry(norm_tokens.clone()).or_insert(Vec::new());
        println!("Number of queries : {}", queries.len());
        // if enough example, create a new template
        if queries.len() >= MINIMUM_QUERIES_FOR_LEARNING {
            let mut injections = Vec::new();
            let mut last = false;
            for i in 0..pruned_tokens.0.len() {
                let t = &pruned_tokens.0[i];
                let mut current = false;
                for q in queries.iter() {
                    if &q.0[i] != t {
                        println!("Injection point : {}",i);
                        injections.push(i);
                        current = true;
                        assert!(!last);
                        last = true;
                        break;
                    }
                }
                last = current;
            }
            let t = Template::new(&tokens, injections);
            println!("{:?}", t);
            self.templates.push(t);
            self.unmatched_queries.clear();
        } else {
            queries.push(pruned_tokens);
        }
    }

}
