use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct VocabularyEntry {
    word: String,
    doc_id: Vec<usize>,
}
impl VocabularyEntry {
    pub fn width_doc_id(word: String, id: usize) -> VocabularyEntry {
        VocabularyEntry {
            word,
            doc_id: vec![id],
        }
    }
    pub fn push_id(&mut self, doc_id: usize) {
        self.doc_id.push(doc_id);
    }
    pub fn word(&self) -> &String {
        &self.word
    }
    pub fn doc_id(&self) -> &Vec<usize> {
        &self.doc_id
    }
    pub fn push_ids(&mut self, ids: &Vec<usize>) {
        for id in ids {
            self.push_id(*id);
        }
    }
}
impl Ord for VocabularyEntry {
    fn cmp(&self, other: &VocabularyEntry) -> Ordering {
        self.word.cmp(&other.word)
    }
}
impl PartialOrd for VocabularyEntry {
    fn partial_cmp(&self, other: &VocabularyEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for VocabularyEntry {
    fn eq(&self, other: &VocabularyEntry) -> bool {
        self.word == other.word
    }
}
impl Eq for VocabularyEntry {}
