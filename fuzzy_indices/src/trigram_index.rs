use crate::trigram_index_entry::Trigram;
use crate::trigram_index_entry::TrigramIndexEntry;
use serde::{Deserialize, Serialize};

// trigram index is a struct that contains vectors of trigrams

#[derive(Serialize, Deserialize)]
pub struct TrigramIndex {
    entries: Vec<TrigramIndexEntry>,
    document_entries: Vec<Vec<TrigramIndexEntry>>,
}

// trigram index can be created with empty constructor
impl TrigramIndex {
    pub fn new() -> TrigramIndex {
        TrigramIndex {
            entries: Vec::new(),
            document_entries: Vec::new(),
        }
    }
}

// members of trigram index struct can be accessed read-only
impl TrigramIndex {
    pub fn entries(&self) -> &Vec<TrigramIndexEntry> {
        &self.entries
    }
    pub fn document_entries(&self) -> &Vec<Vec<TrigramIndexEntry>> {
        &self.document_entries
    }
}

// a document or trigram can be added
impl TrigramIndex {
    pub fn add_document(&mut self, trigram: TrigramIndexEntry) {
        self.document_entries.push(Vec::new());
    }
    pub fn add_trigram(&mut self, trigram: TrigramIndexEntry, doc_id: usize) {
        self.document_entries[doc_id].push(trigram);
    }
    pub fn add_word(&mut self, word: &str, word_position: usize) {
        let sign = '$';
        let chars: Vec<char> = word.chars().collect();
        if chars.len() < 2 {
            return;
        }
        for i in 0..chars.len() {
            let trigram = if i == 0 {
                Trigram::new(sign, chars[i], chars[i + 1])
            } else if i == chars.len() - 1 {
                 Trigram::new(chars[i-1], chars[i], sign)
            } else {
                Trigram::new(chars[i-1], chars[i], chars[i+1])
            };
            let entry: TrigramIndexEntry = TrigramIndexEntry::from_occurence(trigram, word_position);
            self.entries.push(entry);
        }
    }
}

// trigram index can be deduped
impl TrigramIndex {
    fn dedup(&mut self) {
        self.entries.sort();
        let set: Vec<_> = self.entries.drain(..).collect();
        for entry in set {
            if let Some(last) = self.entries.last_mut() {
                if last == &entry {
                    last.add_word(entry.word_position()[0]);
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
        self.dedup();
    }
}
