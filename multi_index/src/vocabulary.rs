use crate::coordinate_entry::CoordinateEntry;
use crate::vocabulary_entry::VocabularyEntry;

pub struct Vocabulary {
    doc_id: Vec<String>,
    single_word_index: Vec<VocabularyEntry>,
    coordinate_index: Vec<CoordinateEntry>,
    double_word_index: Vec<VocabularyEntry>,
}

impl Vocabulary {
    pub fn new() -> Vocabulary {
        Vocabulary {
            doc_id: Vec::new(),
            single_word_index: Vec::new(),
            coordinate_index: Vec::new(),
            double_word_index: Vec::new(),
        }
    }

    pub fn doc_id(&mut self) -> &mut Vec<String> {
        &mut self.doc_id
    }
    pub fn single_word_index(&mut self) -> &mut Vec<VocabularyEntry> {
        &mut self.single_word_index
    }

    pub fn get_doc_id(&self) -> &Vec<String> {
        &self.doc_id
    }
    pub fn get_single_word_index(&self) -> &Vec<VocabularyEntry> {
        &self.single_word_index
    }

    pub fn coordinate_index(&mut self) -> &mut Vec<CoordinateEntry> {
        &mut self.coordinate_index
    }
    pub fn get_coordinate_index(&self) -> &Vec<CoordinateEntry> {
        &self.coordinate_index
    }

    pub fn double_word_index(&mut self) -> &mut Vec<VocabularyEntry> {
        &mut self.double_word_index
    }
    pub fn get_double_word_index(&self) -> &Vec<VocabularyEntry> {
        &self.double_word_index
    }

    pub fn len(&self) -> usize {
        self.doc_id.len()
    }
    pub fn join(&mut self) {
        self.single_word_index.sort();
        let set: Vec<_> = self.single_word_index.drain(..).collect();
        for entry in set {
            if let Some(last) = self.single_word_index.last_mut() {
                if last == &entry {
                    last.push_id(entry.doc_id()[0]);
                    continue;
                }
            }
            self.single_word_index.push(entry);
        }
        for entry in self.single_word_index() {
            entry.dedup();
        }
    }

    pub fn join_coord(&mut self) {
        self.coordinate_index().sort();
        let set: Vec<_> = self.coordinate_index().drain(..).collect();
        for entry in set {
            if let Some(last) = self.coordinate_index().last_mut() {
                if last == &entry {
                    last.add_entry(&entry.doc_id()[0]);
                    continue;
                }
            }
            self.coordinate_index().push(entry);
        }
        for entry in self.coordinate_index() {
            entry.join();
        }
    }

    pub fn join_double(&mut self) {
        self.double_word_index.sort();
        let set: Vec<_> = self.double_word_index.drain(..).collect();
        for entry in set {
            if let Some(last) = self.double_word_index.last_mut() {
                if last == &entry {
                    last.push_id(entry.doc_id()[0]);
                    continue;
                }
            }
            self.double_word_index.push(entry);
        }
        for entry in self.double_word_index() {
            entry.dedup();
        }
    }
}
