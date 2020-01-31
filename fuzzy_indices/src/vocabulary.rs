use crate::inverted_index::InvertedIndex;
use crate::inverted_index_entry::InvertedIndexEntry;
use crate::trigram_index::TrigramIndex;
use crate::permutation_index::PermutationIndex;
use crate::utils;
use rust_stemmers::{Algorithm, Stemmer};

use std::fs;
use std::process;

const ESCAPE_CHARS: [char; 25] = [
    ' ', ',', '.', ':', ';', '|', '/', '(', ')', '{', '}', '[', ']', '"', '\'', '*', '\n', '\t',
    '&', '#', '-', '$', '#', '!', '?',
];

// vocabulary contains doc id (vector of file names) and all the indices that are needed
pub struct Vocabulary {
    doc_id: Vec<String>,
    inverted_index: InvertedIndex,
    trigram_index: TrigramIndex,
    permutation_index: PermutationIndex,
}

// vocabulary has empty constructor
impl Vocabulary {
    pub fn new() -> Vocabulary {
        Vocabulary {
            doc_id: Vec::new(),
            inverted_index: InvertedIndex::new(),
            trigram_index: TrigramIndex::new(),
            permutation_index: PermutationIndex::new(),
        }
    }
}

// we can access class members as mutable or immutable
impl Vocabulary {
    pub fn doc_id(&mut self) -> &mut Vec<String> {
        &mut self.doc_id
    }
    pub fn get_doc_id(&self) -> &Vec<String> {
        &self.doc_id
    }
    pub fn inverted_index(&mut self) -> &mut InvertedIndex {
        &mut self.inverted_index
    }
    pub fn get_inverted_index(&self) -> &InvertedIndex {
        &self.inverted_index
    }
    pub fn trigram_index(&mut self) -> &mut TrigramIndex {
        &mut self.trigram_index
    }
    pub fn get_trigram_index(&self) -> &TrigramIndex {
        &self.trigram_index
    }
    pub fn permutation_index(&mut self) -> &mut PermutationIndex {
        &mut self.permutation_index
    }
    pub fn get_permutation_index(&self) -> &PermutationIndex {
        &self.permutation_index
    }
}

impl Vocabulary {
    pub fn fill_from_file(&mut self, input_dir: &str) {
        let en_stemmer: Stemmer = Stemmer::create(Algorithm::English);
        for entry in utils::read_dir(input_dir) {
            if let Ok(entry) = entry {
                if let Some(contents) = utils::read_file(&entry) {
                    self.doc_id()
                        .push(entry.path().to_str().unwrap().to_owned());
                    let doc_id = self.doc_id().len() - 1;
                    self.inverted_index().add_document();
                    for word in
                        contents.split(|c: char| c.is_numeric() || ESCAPE_CHARS.contains(&c))
                    {
                        // static word: String = en_stemmer.stem(&word.to_lowercase()).into_owned();
                        self.inverted_index().add_word(
                            InvertedIndexEntry::from_first_occurence(word.to_owned(), doc_id),
                            doc_id,
                        );
                    }
                }
            }
        }
        self.inverted_index().process_entries();
    }
    pub fn fill_trigram_index(&mut self) {
        let entries = self.inverted_index().entries();
        let mut trigram_index = TrigramIndex::new();
        let mut permutation_index = PermutationIndex::new();
        for (i, entry) in entries.iter().enumerate() {
            trigram_index.add_word(entry.word(), i);
            permutation_index.add_word(entry.word(), i);
        }
        self.trigram_index = trigram_index;
        self.permutation_index = permutation_index;

        self.trigram_index().process_entries();
        self.permutation_index().process_entries();
    }
}

impl Vocabulary {
    pub fn save(&self, output_dir: &str) {
        match fs::create_dir(output_dir) {
            Ok(_) => {
                const DOC_ID_OUTPUT: &str = "doc_id.json";
                let doc_id_output = format!("{}/{}", output_dir, DOC_ID_OUTPUT);
                if let Ok(result) = serde_json::to_string(&self.doc_id) {
                    utils::write(&doc_id_output, &result);
                }

                const SINGLE_WORD_OUTPUT: &str = "single_word_index.json";
                let single_word_output = format!("{}/{}", output_dir, SINGLE_WORD_OUTPUT);
                if let Ok(result) = serde_json::to_string(&self.inverted_index) {
                    utils::write(&single_word_output, &result);
                }

                const TRIGRAM_INDEX_OUTPUT: &str = "trigram_index.json";
                let double_word_index = format!("{}/{}", output_dir, TRIGRAM_INDEX_OUTPUT);
                if let Ok(result) = serde_json::to_string(&self.get_trigram_index()) {
                    utils::write(&double_word_index, &result);
                }

                const PERMUTATION_INDEX_OUTPUT: &str = "permutation_index.json";
                let permutation_index = format!("{}/{}", output_dir, PERMUTATION_INDEX_OUTPUT);
                if let Ok(result) = serde_json::to_string(&&self.get_permutation_index()) {
                    utils::write(&permutation_index, &result);
                }
            }
            Err(_) => {
                println!("The output dir does not exist");
                process::exit(0);
            }
        }
    }
}
