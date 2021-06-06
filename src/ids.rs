//use std::sync::RwLock TODO
use std::collections::HashMap;
use std::fmt;

use crate::template::Template;
use crate::tokens::*;

const MINIMUM_QUERIES_FOR_LEARNING: usize = 3;

#[derive(Debug)]
pub enum ReqFate {
    Unknown,
    Pass(String),
    Del(Vec<String>)
}

impl fmt::Display for ReqFate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReqFate::Unknown => write!(f, "Unknown: no template found"),
            ReqFate::Pass(t) => write!(f, "Pass: validated by template {}", t),
            ReqFate::Del(t) => write!(f, "Del: invalidated by templates {:?}", t),
        }
    }
}


pub struct Ids {
    templates: Vec<Template>,
    unmatched_queries: HashMap<NormalizedTokens, Vec<RawTokens>> // index: normalized. value. not normalized
}

impl Ids {
    pub fn new() -> Ids {
        Ids { templates: Vec::new(), unmatched_queries: HashMap::new() }
    }

    pub fn handle_req(&mut self, req: &str) -> ReqFate {
        println!("Received queries: {}",req);
        let out = self.verify_req(req);
        match out {
            ReqFate::Unknown => self.learn(tokenize(req)),
            _ => ()
        };
        self.clean();
        out
    }

    fn verify_req(&self, req: &str) -> ReqFate {
        let tokens = tokenize(req);
        let norm_tokens = normalize(tokens.clone());
        let mut invalid_templates = Vec::new();
        for t in self.templates.iter() {
            if t.is_match(req) {
                // println!("Match with {}", t);
                invalid_templates.push(format!("{}",t));
                if t.is_legitimate(&norm_tokens) {
                    return ReqFate::Pass(format!("{}",t));
                }
            }
        }
        match invalid_templates.is_empty() {
            false => ReqFate::Del(invalid_templates),
            true => {
                ReqFate::Unknown
            }
        }
    }

    fn clean(&mut self) {
        // remove unmatched_queries that have been invalidated since
        let retain = |_: &NormalizedTokens,v: &mut Vec<RawTokens>| {
            let result = self.verify_req(&format!("{}", v.last().unwrap()));
            match result {
                ReqFate::Del(_) => {
                    println!("Query {} is removed from unmatched queries ({})", v.last().unwrap(), &result);
                    false
                },
                _ => true
            }
        };
        let mut clone = self.unmatched_queries.clone();
        clone.retain(retain);
        self.unmatched_queries = clone;
    }

    fn learn(&mut self, tokens: RawTokens) {
        let norm_tokens = normalize(tokens.clone());
        let queries = self.unmatched_queries.entry(norm_tokens.clone()).or_insert(Vec::new());
        // if enough example, create a new template
        if queries.len() >= MINIMUM_QUERIES_FOR_LEARNING {
            let pruned_tokens = prune(tokens.clone());
            let mut injections = Vec::new();
            // let mut last = false;
            for i in 0..pruned_tokens.0.len() {
                let t = &pruned_tokens.0[i];
                // let mut current = false;
                for q in queries.iter() {
                    let q = prune(q.clone());
                    if &q.0[i] != t {
                        injections.push(i);
                        // current = true;
                        // assert!(!last);
                        // last = true;
                        break;
                    }
                }
                // last = current;
            }
            let t = Template::new(&tokens, injections);
            println!("New template: {}", t);
            self.templates.push(t);
            self.unmatched_queries.remove_entry(&norm_tokens);
        } else {
            queries.push(tokens);
        }
    }

}
