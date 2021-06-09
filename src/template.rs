use std::fmt;
use regex::Regex;
use sqlparser::tokenizer::Token;
use crate::tokens::{normalize_once,NormalizedTokens,RawTokens,is_whitespace};

#[derive(Debug, Clone)]
pub struct Template {
    printable: String,
    re: Regex, // a compiled regex
    norm_length: usize,
    inj_tokens: Vec<(Token,usize)>, // a list of token that represent valid injections. The indexes must be increasing
}

impl PartialEq for Template {
    fn eq(&self, other: &Self) -> bool {
        self.printable == other.printable
    }
}

impl Eq for Template {}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.printable)
    }
}

impl Template {
    pub fn new(template_tokens: &RawTokens, mut inj_indexes: Vec<usize>) -> Template {
        // construct the regex from the tokens
        let mut regex = String::from("^");
        inj_indexes.reverse();
        let mut inj_tokens = Vec::new();
        let mut next_inj_index = inj_indexes.pop();
        let mut new_index = 0;
        let mut printable = String::new();
        for i in 0..template_tokens.0.len() {
            let t = &template_tokens.0[i];
            if next_inj_index.is_some() && !is_whitespace(t) && next_inj_index.unwrap()==new_index {
                // injection generally don't include the quotes
                let (l,r) = match t {
                    Token::SingleQuotedString(_) => (String::from("'"),String::from("'")),
                    Token::NationalStringLiteral(_) => (String::from("N'"),String::from("'")),
                    Token::HexStringLiteral(_) => (String::from("X'"),String::from("'")),
                    Token::Word(w) => match w.quote_style {

                        Some(c) => (String::from(c),String::from(c)),
                        _ => (String::new(), String::new())
                        },
                    _ => (String::new(), String::new())
                };

                printable += &(l.clone() + "[[INJECTION]]" + &r);
                regex += &(l + ".*" + &r);
                inj_tokens.push((normalize_once(t.clone()).unwrap(), new_index)); // unwrap is safe due to condition
                next_inj_index = inj_indexes.pop();
            }
            else {
                regex += &regex::escape(&format!("{}", t));
                printable += &format!("{}", t);
            }
            if !is_whitespace(t) {
                new_index += 1;
            }
        }
        regex += "$";
        // println!("Regex: {}", regex);
        Template { printable, re: Regex::new(&regex).unwrap(), norm_length: new_index, inj_tokens }
    }

    pub fn is_match(&self, s: &str) -> bool {
        self.re.is_match(s)
    }

    pub fn is_legitimate(&self, input: &NormalizedTokens) -> bool {
        if input.0.len() != self.norm_length { // wrong size
            false
        } else {
            for (t1,u) in self.inj_tokens.iter() { // only check the injections
                let t2 = &input.0[*u];
                if t1 != t2 {
                    // println!("Injection detected ! Expected: {:?}. Received: {:?}.", t1, t2);
                    return false
                }
            }
            true
        }
    }

}
