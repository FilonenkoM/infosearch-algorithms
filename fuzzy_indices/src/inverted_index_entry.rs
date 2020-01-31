use std::cmp::Ord;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

// inverted index entry contains word and document ids of documents, that contains this word
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvertedIndexEntry {
    word: String,
    documents: Vec<usize>,
}

// inverted index entry can be create by word, word and one id, word and vec of document ids
impl InvertedIndexEntry {
    pub fn from_word(word: String) -> InvertedIndexEntry {
        InvertedIndexEntry {
            word,
            documents: Vec::new()
        }
    }
    pub fn from_first_occurence(word: String, id: usize) -> InvertedIndexEntry {
        InvertedIndexEntry {
            word, 
            documents: vec![id]
        }
    }
    pub fn from_list(word: String, documents: Vec<usize>) -> InvertedIndexEntry {
        InvertedIndexEntry {
            word,
            documents
        }
    }
}

// members of inverted index entry can be accessed read-only
impl InvertedIndexEntry {
    pub fn word(&self) -> &str {
        &self.word
    }
    pub fn documents(&self) -> &Vec<usize> {
        &self.documents
    }
}

// document can be added to the entry. Entries can be sorted or deleted
impl InvertedIndexEntry {
    pub fn dedup(&mut self) {
        self.documents.dedup()
    }
    pub fn sort(&mut self) {
        self.documents.sort()
    }
    pub fn push(&mut self, id: usize) {
        self.documents.push(id)
    }
}

// inverted index entries can be compared and sorted
impl Ord for InvertedIndexEntry {
    fn cmp(&self, other: &InvertedIndexEntry) -> Ordering {
        self.word.cmp(&other.word)
    }
}
impl PartialOrd for InvertedIndexEntry {
    fn partial_cmp(&self, other: &InvertedIndexEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for InvertedIndexEntry {
    fn eq(&self, other: &InvertedIndexEntry) -> bool {
        self.word == other.word
    }
}
impl Eq for InvertedIndexEntry {}