use crate::token::Token;
use crate::utils;
use maplit;
use std::collections::BTreeMap;
use std::fmt;

type Term = String;
type Occurences = Vec<usize>;

#[derive(Debug)]
pub struct Vocabulary {
    map: BTreeMap<Term, Occurences>,
}

// constructors
impl Vocabulary {
    pub fn from_map(map: BTreeMap<Term, Occurences>) -> Vocabulary {
        Vocabulary { map }
    }
    pub fn new() -> Vocabulary {
        Self::from_map(BTreeMap::new())
    }
    pub fn from_token(token: Token) -> Vocabulary {
        Self::from_map(btreemap! {String::from(token.term()) => vec![*token.doc_id()]})
    }
}

// push the token
impl Vocabulary {
    pub fn add_token(&mut self, token: Token) {
        if let Some(entry) = self.map.get_mut(token.term()) {
            if !entry.contains(token.doc_id()) {
                entry.push(*token.doc_id());
            }
        } else {
            self.map
                .insert(String::from(token.term()), vec![*token.doc_id()]);
        }
    }
}

impl fmt::Display for Vocabulary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (key, value) in &self.map {
            result.push_str(key);
            result.push_str(": ");
            for id in value {
                result.push_str(&format!("{}, ", id));
            }
            result.push('\n');
        }
        write!(f, "{}", &result)
    }
}

// save to the file
impl Vocabulary {
    pub fn save(&self, file: &str) {
        utils::write(file, &format!("{}", self));
    }
}
