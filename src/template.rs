
use regex::Regex;
use sqlparser::tokenizer::Token;

pub struct Template {
    re: Regex, // a compiled regex
    tok: Vec<(Token,bool)>
}

impl Template {
    pub fn new(template_parts: &Vec<&str>, tok: Vec<(Token,bool)>) -> Template
    {
        assert!(template_parts.len()>0);
        let mut iter = template_parts.iter();
        let mut regex = "^".to_string() + &regex::escape(iter.next().unwrap());
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
        println!("{}",regex);
        Template { re: Regex::new(&regex).unwrap(), tok }
    }

    pub fn is_match(&self, s: &str) -> bool
    {
        self.re.is_match(s)
    }

    pub fn is_legitimate(&self, input: &Vec<Token>) -> bool
    {
        if dbg!(self.tok.len() != input.len())
        {
            false
        }
        else
        {
            println!("{:?}", self.tok);
            println!("{:?}", input);
            for t in 1..self.tok.len()
            {
                let (t1,inj) = &self.tok[t];
                let t2 = &input[t];
                match inj {
                    true => match (t1,t2)
                        {
                            (Token::Word(w1),Token::Word(w2)) =>
                                if dbg!(w1.keyword != w2.keyword)
                                {
                                    return false // only compare the type of the keyword and not the value
                                },
                            (Token::Number(_,_),Token::Number(_,_)) => (), // both numbers, don't check the value
                            (Token::Char(_),Token::Char(_)) => (), // both chars, don't check the value
                            (Token::SingleQuotedString(_),Token::SingleQuotedString(_)) => (), // both strings, don't check the value
                            (Token::NationalStringLiteral(_),Token::NationalStringLiteral(_)) => (),
                            (Token::HexStringLiteral(_),Token::HexStringLiteral(_)) => (),
                            (o1,o2) =>
                                if dbg!(o1 != o2)
                                {
                                    return false // different type
                                }
                        }
                    false => if dbg!(t1 != t2)
                        {
                            return false
                        }
                }
            }
            true
        }
    }
}
