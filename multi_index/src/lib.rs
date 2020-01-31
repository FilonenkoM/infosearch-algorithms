mod coordinate_entry;
mod vocabulary;
mod vocabulary_entry;
use self::coordinate_entry::CoordinateEntry;
use self::coordinate_entry::DocCoordEntry;
use self::vocabulary::Vocabulary;
use self::vocabulary_entry::VocabularyEntry;
use serde_json::Result;
use std::fs;
use std::fs::DirEntry;
use std::fs::ReadDir;
use std::io::ErrorKind;
use std::process;

pub fn build(input: &str) -> Vocabulary {
    let mut v = Vocabulary::new();
    single_word_index(&mut v, input);
    v
}

fn single_word_index(vocabulary: &mut Vocabulary, input: &str) {
    let mut total_words = 0;
    let mut size = 0;
    let mut previous_word: Option<String> = None;
    for file in read_dir(input) {
        let file = file.unwrap();
        if let Some(contents) = read_file(&file) {
            size += fs::metadata(&file.path()).unwrap().len();
            vocabulary
                .doc_id()
                .push(file.path().to_str().unwrap().to_owned());
            for (index, word) in split_into_words(&contents).iter().enumerate() {
                if word.len() < 1 {
                    continue;
                }
                total_words += 1;
                let word = word.trim().to_string();
                let doc_id = vocabulary.doc_id().len() - 1;

                if let Some(prev) = previous_word {
                    let resulting_word = format!("{} {}", prev, word);
                    vocabulary
                        .double_word_index()
                        .push(VocabularyEntry::width_doc_id(resulting_word, doc_id));
                }
                previous_word = Some(word.to_string());

                previous_word = Some(word.to_string());
                vocabulary
                    .single_word_index()
                    .push(VocabularyEntry::width_doc_id(word.clone(), doc_id));
                vocabulary
                    .coordinate_index()
                    .push(CoordinateEntry::with_doc_entry(
                        word.clone(),
                        DocCoordEntry::with_occurence(doc_id, index),
                    ));
            }
        }
    }
    vocabulary.join();
    vocabulary.join_coord();
    vocabulary.join_double();
}

pub fn save(output_dir: &str, vocabulary: &Vocabulary) {
    match fs::create_dir(output_dir) {
        Ok(_) => {
            const DOC_ID_OUTPUT: &str = "doc_id.json";
            let doc_id_output = format!("{}/{}", output_dir, DOC_ID_OUTPUT);
            if let Ok(result) = serde_json::to_string(&vocabulary.get_doc_id()) {
                write(&doc_id_output, &result);
            }

            const SINGLE_WORD_OUTPUT: &str = "single_word_index.json";
            let single_word_output = format!("{}/{}", output_dir, SINGLE_WORD_OUTPUT);
            if let Ok(result) = serde_json::to_string(&vocabulary.get_single_word_index()) {
                write(&single_word_output, &result);
            }

            const DOUBLE_WORD_INDEX: &str = "double_word_index.json";
            let double_word_index = format!("{}/{}", output_dir, DOUBLE_WORD_INDEX);
            if let Ok(result) = serde_json::to_string(&vocabulary.get_double_word_index()) {
                write(&double_word_index, &result);
            }

            const COORDINATE_INDEX: &str = "coordinate_index.json";
            let coordinate_index = format!("{}/{}", output_dir, COORDINATE_INDEX);
            if let Ok(result) = serde_json::to_string(&vocabulary.get_coordinate_index()) {
                write(&coordinate_index, &result);
            }
        }
        Err(_) => {
            println!("The output dir does not exist");
            process::exit(0);
        }
    }
}

fn split_into_words(contents: &str) -> Vec<String> {
    contents
        .split(|c: char| !c.is_alphabetic())
        .map(|s| s.to_lowercase())
        .collect()
}

fn read_file(file: &DirEntry) -> Option<String> {
    match fs::read_to_string(file.path()) {
        Ok(result) => Some(result),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!(
                    "File {} not found. Maybe, it was deleted during the process",
                    file.path().display()
                ),
                ErrorKind::PermissionDenied => println!(
                    "Not enough permissions to read the {}",
                    file.path().display()
                ),
                ErrorKind::InvalidData => println!(
                    "The file {} contains invalid Unicode",
                    file.path().display()
                ),
                _ => println!("Cannot read the file {}", file.path().display()),
            }
            println!("This file will be skipped");
            None
        }
    }
}

fn write(output: &str, contents: &str) -> bool {
    match fs::write(output, contents) {
        Ok(_) => true,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("Incorrect path"),
                ErrorKind::PermissionDenied => {
                    println!("Not enough permissions to write to this file")
                }
                _ => println!("Can not write to this file"),
            }
            false
        }
    }
}

fn read_dir(dir: &str) -> ReadDir {
    match fs::read_dir(dir) {
        Ok(result) => result,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => println!("Input directory does not exist"),
                ErrorKind::PermissionDenied => {
                    println!("Not enough permissions to read from this directory")
                }
                _ => println!("Cannot read files from the given directory"),
            }
            process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    const INPUT_DIR: &str = "files";
    const OUTPUT_DIR: &str = "vocabulary";

    #[test]
    fn bench() {
        let start = Instant::now();
        let v = build(INPUT_DIR);
        save(OUTPUT_DIR, &v);

        let duration = start.elapsed();
        println!("indexing completed in {:?}", duration);
    }
}
