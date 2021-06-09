//use std::sync::RwLock TODO
use std::collections::HashMap;
use std::fmt;

use crate::template::Template;
use crate::tokens::*;

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
    templates: HashMap<NormalizedTokens, (Vec<String>, Template)>
}

impl Ids {
    pub fn new() -> Ids {
        Ids { templates: HashMap::new() }
    }

    pub fn handle_req(&mut self, req: &str) -> ReqFate {
        println!("Received queries: {}",req);
        let out = self.verify_req(req);
        match out {
            ReqFate::Unknown => self.learn(req.to_string()),
            _ => ()
        };
        out
    }

    fn verify_req(&self, req: &str) -> ReqFate {
        let tokens = tokenize(req);
        let norm_tokens = normalize(tokens.clone());
        let mut invalid_templates = Vec::new();
        for (_,t) in self.templates.iter() {
            if t.1.is_match(req) {
                // println!("Match with {}", t);
                invalid_templates.push(format!("{}",t.1));
                if t.1.is_legitimate(&norm_tokens) {
                    return ReqFate::Pass(format!("{}",t.1));
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

    fn clean(&mut self, t: Template) {
        // remove unmatched_queries that have been invalidated since
        let retain = |req: &String| {
            if t.is_match(&req) {
                let norm_tokens = normalize(tokenize(&req));
                if !t.is_legitimate(&norm_tokens) {
                    println!("Removed query: {}", req);
                    return false
                }
            };
            true
        };
        for (_,(queries,other_t)) in self.templates.iter_mut() {
            let size_before = queries.len();
            if &t != other_t {
                queries.retain(retain);
            }
            if size_before != queries.len() {
                if !queries.is_empty() {
                    // update queries with removed malicious queries
                    println!("Template update");
                    *other_t = Ids::create_template_from_queries(queries);
                } else {
                    println!("Template removed!");
                }
            }
        }
        // remove templates without queries
        self.templates.retain(|_,(queries,_)| queries.is_empty());
    }

    fn create_template_from_queries(queries: &Vec<String>) -> Template {
        assert!(!queries.is_empty());
        let tokens = tokenize(&queries.last().unwrap());
        let pruned_tokens = prune(tokens.clone());
        let mut injections = Vec::new();
        // let mut last = false;
        for i in 0..pruned_tokens.0.len() {
            let t = &pruned_tokens.0[i];
            // let mut current = false;
            for q in queries.iter() {
                let q = prune(tokenize(&q));
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

        Template::new(&tokens, injections)
    }

    fn learn(&mut self, query: String) {
        let mut clean_with_template = None;
        let tokens = tokenize(&query);
        let norm_tokens = normalize(tokens.clone());
        match self.templates.get_mut(&norm_tokens) {
            Some((queries, old_template)) => {
                queries.push(query);
                let t = Ids::create_template_from_queries(queries);
                println!("New template: {}", t);
                if &t != old_template {
                    println!("Template update");
                    clean_with_template = Some(t.clone());
                    *old_template = t;
                }
            },
            None => {
                let t = Template::new(&tokens, Vec::new());
                println!("New template: {}", t);
                self.templates.insert(norm_tokens, (vec![query], t));
            }
        };
        match clean_with_template {
            None => (),
            Some(t) => self.clean(t)
        }
    }

}
