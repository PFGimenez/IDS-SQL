use regex::Regex;
use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::Whitespace;
use crate::tokens::{normalize_once,NormalizedTokens,RawTokens};
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
        let mut start : usize = 0;
        let mut regex = String::from("^");
        for t in inj_indexes.iter() {
            let mut query = String::from("");
            assert!(*t>=start); // verify the order of the inj_indexes
            for i in start..*t {
                query += &format!("{}", template_tokens.0[i]);
            }
            start = t + 1;
            regex += &regex::escape(&query);
            regex += ".*";
        }
        let mut query = String::from("");
        assert!(template_tokens.0.len()>=start);
        for i in start..template_tokens.0.len() {
            query += &format!("{}", template_tokens.0[i]);
        }
        regex += &regex::escape(&query);
        regex += "^";
        // println!("Regex: {}", regex);

        // convert injections indexes from real tokens to normalized tokens
        let mut norm_index = 0;
        let mut curr_index = 0;
        let mut norm_inj_indexes = Vec::new();
        inj_indexes.reverse(); // first tokens at the end so we can pop
        match inj_indexes.pop() {
            Some(mut inj_index) =>
            {
                loop {
                    // println!("{} {}", inj_index, curr_index);
                    if inj_index == curr_index {
                        // norm_inj_indexes.push((normalize_once(template_tokens.0[curr_index].clone()).expect("Injection is a whitespace !"),norm_index));
                        norm_inj_indexes.push((normalize_once(template_tokens.0[curr_index].clone()).expect("Injection is a whitespace !"),curr_index));
                        // println!("New index: {}",norm_index);
                        match inj_indexes.pop() {
                            Some(next_inj_index) => inj_index = next_inj_index,
                            None => break
                        }
                    }
                    match template_tokens.0[curr_index] {
                        Token::Whitespace(Whitespace::Space) => (),
                        Token::Whitespace(Whitespace::Newline) => (),
                        Token::Whitespace(Whitespace::Tab) => (),
                        _ => norm_index += 1
                    };
                    curr_index += 1;
                }
            },
            None => () // no injection at all
        }

        Template { re: Regex::new(&regex).unwrap(), inj_tokens: norm_inj_indexes }
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
