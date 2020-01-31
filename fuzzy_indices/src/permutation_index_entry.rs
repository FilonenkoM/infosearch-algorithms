use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cell::Cell;

#[derive(Serialize, Deserialize)]
pub struct PermutationIndexEntry{
    permutation: String,
    word_positions: Vec<usize>
}

impl PermutationIndexEntry {
    pub fn from_permutation(permutation: String) -> PermutationIndexEntry {
        PermutationIndexEntry {
            permutation,
            word_positions: Vec::new(),
        }
    }
    pub fn from_occurence(permutation: String, word_position: usize) -> PermutationIndexEntry {
        PermutationIndexEntry {
            permutation,
            word_positions: vec![word_position]
        }
    }
    pub fn add_word(&mut self, word_position: usize) {
        self.word_positions.push(word_position);
    }
    pub fn dedup(&mut self) {
        self.word_positions.dedup();
    }
}

impl PermutationIndexEntry {
    pub fn word_position(&self) -> &Vec<usize> {
        &self.word_positions
    }
}

// trigram index entries can be compared
impl Ord for PermutationIndexEntry {
    fn cmp(&self, other: &PermutationIndexEntry) -> Ordering {
        self.permutation.cmp(&other.permutation)
    }
}
impl<'a> PartialOrd for PermutationIndexEntry {
    fn partial_cmp(&self, other: &PermutationIndexEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> PartialEq for PermutationIndexEntry {
    fn eq(&self, other: &PermutationIndexEntry) -> bool {
        self.permutation == other.permutation
    }
}
impl<'a> Eq for PermutationIndexEntry {}