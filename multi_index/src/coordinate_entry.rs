use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoordinateEntry {
    word: String,
    doc_id: Vec<DocCoordEntry>,
}

impl CoordinateEntry {
    pub fn word(&self) -> &String {
        &self.word
    }
    pub fn doc_id(&self) -> &Vec<DocCoordEntry> {
        &self.doc_id
    }
    pub fn new(word: String) -> CoordinateEntry {
        CoordinateEntry {
            word,
            doc_id: Vec::new(),
        }
    }
    pub fn add_doc(&mut self, doc: usize) {
        self.doc_id.push(DocCoordEntry::new(doc));
    }

    pub fn get_doc_id(&self) -> &Vec<DocCoordEntry> {
        &self.doc_id
    }

    pub fn add_entry(&mut self, entry: &DocCoordEntry) {
        self.doc_id.push(DocCoordEntry::new(entry.id()));
        for occurence in entry.occurences() {
            self.doc_id.last_mut().unwrap().add(*occurence);
        }
    }

    pub fn join(&mut self) {
        self.doc_id.sort();
        let set: Vec<_> = self.doc_id.drain(..).collect();
        for entry in set {
            if let Some(last) = self.doc_id.last_mut() {
                if last == &entry {
                    last.add(entry.occurences()[0]);
                    continue;
                }
            }
            self.doc_id.push(entry);
        }
    }

    pub fn with_doc_entry(word: String, entry: DocCoordEntry) -> CoordinateEntry {
        CoordinateEntry {
            word,
            doc_id: vec![entry],
        }
    }
}

impl fmt::Debug for CoordinateEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ word/: {}, doc_id: {:?} }} \n", self.word, self.doc_id)
    }
}

impl fmt::Display for CoordinateEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ word: {}, doc_id: {:?} }} \n", self.word, self.doc_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DocCoordEntry {
    id: usize,
    occurences: Vec<usize>,
}
impl DocCoordEntry {
    pub fn new(id: usize) -> DocCoordEntry {
        DocCoordEntry {
            id,
            occurences: Vec::new(),
        }
    }
    pub fn add(&mut self, occurence: usize) {
        self.occurences.push(occurence);
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn occurences(&self) -> &Vec<usize> {
        &self.occurences
    }
    pub fn with_occurence(id: usize, occurence: usize) -> DocCoordEntry {
        DocCoordEntry {
            id,
            occurences: vec![occurence],
        }
    }
}

impl fmt::Display for DocCoordEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ id: {}, occurences: {:?} }}", self.id, self.occurences())
    }
}

impl fmt::Debug for DocCoordEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ id: {}, occurences: {:?} }}", self.id, self.occurences())
    }
}

impl Ord for CoordinateEntry {
    fn cmp(&self, other: &CoordinateEntry) -> Ordering {
        self.word.cmp(&other.word)
    }
}
impl PartialOrd for CoordinateEntry {
    fn partial_cmp(&self, other: &CoordinateEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for CoordinateEntry {
    fn eq(&self, other: &CoordinateEntry) -> bool {
        self.word == other.word
    }
}
impl Eq for CoordinateEntry {}

impl Ord for DocCoordEntry {
    fn cmp(&self, other: &DocCoordEntry) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for DocCoordEntry {
    fn partial_cmp(&self, other: &DocCoordEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for DocCoordEntry {
    fn eq(&self, other: &DocCoordEntry) -> bool {
        self.id == other.id
    }
}
impl Eq for DocCoordEntry {}