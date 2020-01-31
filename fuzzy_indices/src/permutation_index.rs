
use crate::trigram_index_entry::Trigram;
use crate::trigram_index_entry::TrigramIndexEntry;
use serde::{Deserialize, Serialize};
use crate::permutation_index_entry::PermutationIndexEntry;

#[derive(Serialize, Deserialize)]
pub struct PermutationIndex {
    entries: Vec<PermutationIndexEntry>,
    document_entries: Vec<Vec<PermutationIndexEntry>>,
}

impl PermutationIndex {
    pub fn new() -> PermutationIndex {
        PermutationIndex {
            entries: Vec::new(),
            document_entries: Vec::new(),
        }
    }
}

impl PermutationIndex {
    pub fn entries(&self) -> &Vec<PermutationIndexEntry> {
        &self.entries
    }
    pub fn document_entries(&self) -> &Vec<Vec<PermutationIndexEntry>> {
        &self.document_entries
    }
}

impl PermutationIndex {
    pub fn add_document(&mut self, permutation: TrigramIndexEntry) {
        self.document_entries.push(Vec::new());
    }
    pub fn add_trigram(&mut self, permutation: PermutationIndexEntry, doc_id: usize) {
        self.document_entries[doc_id].push(permutation);
    }
    pub fn add_word(&mut self, word: &str, word_position: usize) {
        let sign = '$';
        let mut chars: Vec<char> = word.chars().collect();
        if chars.len() < 2 {
            return;
        }
        chars.push('$');
        for i in 0..chars.len() {
            let p = PermutationIndexEntry::from_occurence(chars.iter().collect(), word_position);
            self.entries.push(p);
            let c = chars[i];
            chars.remove(0);
            chars.push(c);
        }
    }
}

impl PermutationIndex {
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
