use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cell::Cell;

#[derive(Serialize, Deserialize)]
pub struct TrigramIndexEntry{
    trigram: Trigram,
    word_positions: Vec<usize>
}

impl TrigramIndexEntry {
    pub fn from_trigram(trigram: Trigram) -> TrigramIndexEntry {
        TrigramIndexEntry {
            trigram,
            word_positions: Vec::new(),
        }
    }
    pub fn from_occurence(trigram: Trigram, word_position: usize) -> TrigramIndexEntry {
        TrigramIndexEntry {
            trigram,
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

impl TrigramIndexEntry {
    pub fn word_position(&self) -> &Vec<usize> {
        &self.word_positions
    }
}

// trigram index entries can be compared
impl Ord for TrigramIndexEntry {
    fn cmp(&self, other: &TrigramIndexEntry) -> Ordering {
        self.trigram.cmp(&other.trigram)
    }
}
impl<'a> PartialOrd for TrigramIndexEntry {
    fn partial_cmp(&self, other: &TrigramIndexEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> PartialEq for TrigramIndexEntry {
    fn eq(&self, other: &TrigramIndexEntry) -> bool {
        self.trigram == other.trigram
    }
}
impl<'a> Eq for TrigramIndexEntry {}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trigram {
    a: char,
    b: char,
    c: char,
}
impl Trigram {
    pub fn new(a: char, b: char, c: char) -> Trigram {
        Trigram {
            a, b, c
        }
    }
}
impl Ord for Trigram {
    fn cmp(&self, other: &Trigram) -> Ordering {
        match self.a.cmp(&other.a) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match self.b.cmp(&other.b) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => match self.c.cmp(&other.c) {
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => Ordering::Equal,
                }
            }
        }
    }
}
impl PartialOrd for Trigram {
    fn partial_cmp(&self, other: &Trigram) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Trigram {
    fn eq(&self, other: &Trigram) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}
impl Eq for Trigram {}