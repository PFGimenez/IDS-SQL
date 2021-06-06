use regex::Regex;
use sqlparser::tokenizer::Token;
use crate::tokens::{normalize_once,NormalizedTokens,RawTokens,is_whitespace};
// use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Template {
    re: Regex, // a compiled regex
    inj_tokens: Vec<(Token,usize)>, // a list of token that represent valid injections. The indexes must be increasing
    // last_usage: SystemTime
}

impl Template {
    pub fn new(template_tokens: &RawTokens, mut inj_indexes: Vec<usize>) -> Template {
        // construct the regex from the tokens
        let mut regex = String::from("^");
        inj_indexes.reverse();
        let mut inj_tokens = Vec::new();
        let mut next_inj_index = inj_indexes.pop();
        let mut new_index = 0;
        for i in 0..template_tokens.0.len() {
            let t = &template_tokens.0[i];
            if next_inj_index.is_some() && !is_whitespace(t) && next_inj_index.unwrap()==new_index {
                regex += ".*";
                inj_tokens.push((normalize_once(t.clone()).unwrap(), new_index)); // unwrap is safe due to condition
                next_inj_index = inj_indexes.pop();
            }
            else {
                regex += &regex::escape(&format!("{}", t));
            }
            if !is_whitespace(t) {
                new_index += 1;
            }
        }
        regex += "$";

        Template { re: Regex::new(&regex).unwrap(), inj_tokens }
    }

    pub fn is_match(&self, s: &str) -> bool {
        self.re.is_match(s)
    }

    pub fn is_legitimate(&self, input: &NormalizedTokens) -> bool {
        for (t1,u) in self.inj_tokens.iter() { // only check the injections
            let t2 = &input.0[*u];
            if t1 != t2 {
                return false
            }
        }
        true
    }

}
