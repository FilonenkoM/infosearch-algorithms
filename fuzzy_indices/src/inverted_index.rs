use crate::inverted_index_entry::InvertedIndexEntry;
use serde::{Deserialize, Serialize};

// Inverted index contains an vector of inverted index entries
#[derive(Serialize, Deserialize, Debug)]
pub struct InvertedIndex {
    entries: Vec<InvertedIndexEntry>,
    document_indices: Vec<Vec<InvertedIndexEntry>>,
}

// inverted index can be created with no params
impl InvertedIndex {
    pub fn new() -> InvertedIndex {
        InvertedIndex {
            entries: Vec::new(),
            document_indices: Vec::new(),
        }
    }
}

// members of the inverted index can be accessed only readonly
impl InvertedIndex {
    pub fn entries(&self) -> &Vec<InvertedIndexEntry> {
        &self.entries
    }
    pub fn document_indices(&self) -> &Vec<Vec<InvertedIndexEntry>> {
        &self.document_indices
    }
}

// a document and entry can be added to the index
impl InvertedIndex {
    pub fn add_document(&mut self) {
        self.document_indices.push(Vec::new());
    }
    pub fn add_word(&mut self, entry: InvertedIndexEntry, index: usize) {
        self.document_indices[index].push(entry);
    }
}

// inverted index entries vector can be serialized to the json string
impl InvertedIndex {
    pub fn to_json(&self) -> String {
        match serde_json::to_string(self.entries()) {
            Ok(_result) => _result,
            Err(err) => panic!(err),
        }
    }
}

// the inverted index entries can be sorted and deduped
impl InvertedIndex {
    fn dedup(&mut self) {
        self.entries.sort();
        let set: Vec<_> = self.entries.drain(..).collect();
        for entry in set {
            if let Some(last) = self.entries.last_mut() {
                if last == &entry {
                    last.push(entry.documents()[0]);
                    continue;
                }
            }
            self.entries.push(entry);
        }
        for entry in &mut self.entries {
            entry.dedup();
        }
    }
    pub fn process_entries(&mut self) {
        for document_entry in &mut self.document_indices {
            document_entry.sort();
            document_entry.dedup();
            self.entries.append(document_entry);
        }
        self.dedup();
    }
}


