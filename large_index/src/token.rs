#[derive(Debug)]
pub struct Token {
    term: String,
    doc_id: usize,
}

// constructor
impl Token {
    pub fn new(term: String, doc_id: usize) -> Token {
        Token { term, doc_id }
    }
}

// seeters / getters
impl Token {
    #[inline] 
    pub fn term(&self) -> &str {
        &self.term
    }
    #[inline] 
    pub fn term_mut(&mut self) -> &mut str {
        &mut self.term
    }
    #[inline] 
    pub fn term_set(&mut self, new_term: String) {
        self.term = new_term
    }
    #[inline] 
    pub fn doc_id(&self) -> &usize {
        &self.doc_id
    }
    #[inline] 
    pub fn doc_id_mut(&mut self) -> &mut usize {
        &mut self.doc_id
    }
    #[inline] 
    pub fn doc_id_set(&mut self, new_doc_id: usize) {
        self.doc_id = new_doc_id
    }
}

// display 
use std::fmt;
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.term, self.doc_id)
    }
}