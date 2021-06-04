use regex::Regex;
use sqlparser::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct Template {
    re: Regex, // a compiled regex
    inj_tok: Vec<(Token,usize)> // a list of token that represent valid injections
}

impl Template {
    pub fn new(template_parts: &Vec<&str>, inj_tok: Vec<(Token,usize)>) -> Template {
        assert_eq!(inj_tok.len() + 1, template_parts.len());
        let mut iter = template_parts.iter();
        let mut regex = String::from("^") + &regex::escape(iter.next().unwrap());
        loop
        {
            // construct the RE in the form : ^ p1 .* p2 .* p3 $
            match iter.next() {
                None => break,
                Some(p) => {
                    regex += ".*";
                    regex += &regex::escape(p);
                }
            }
        }
        regex += "$";
        Template { re: Regex::new(&regex).unwrap(), inj_tok }
    }

    pub fn is_match(&self, s: &str) -> bool {
        self.re.is_match(s)
    }

    pub fn is_legitimate(&self, input: &Vec<Token>) -> bool {
        for t in 1..self.inj_tok.len() { // only check the injections
            let (t1,u) = &self.inj_tok[t];
            let t2 = &input[*u];
            if t1 != t2 {
                return false
            }
        }
        true
    }
}
