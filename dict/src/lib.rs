use std::fs;
use std::fs::ReadDir;
use std::io::ErrorKind;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
mod vocabulary_entry;
use self::vocabulary_entry::VocabularyEntry;

/// enum Algorithm indicates what algorithm has to be used
pub enum Algorithm {
    Greedy,
    GreedyMultithread,
    Quick,
}

/// the struct used to pass statistics of the built vocabulary
pub struct Statistics {
    files: usize,
    entries: usize,
    vocabulary_size: u64
}

impl Statistics {
    pub fn files(&self) -> usize {
        self.files
    }
    pub fn entries(&self) -> usize {
        self.entries
    }
    pub fn vocabulary_size(&self) -> u64 {
        self.vocabulary_size
    }
}

/// The function returns Statistics of built vocabulary 
pub fn vocabulary(input_dir: &str, output_file: &str, algorithm: Algorithm) -> Statistics {
    let input_dir = read_dir(input_dir);
    let (doc_id, vocabulary) = match algorithm {
        Algorithm::Greedy => vocabulary_greedy(input_dir),
        Algorithm::GreedyMultithread => vocabulary_greedy_multithread(input_dir),
        Algorithm::Quick => vocabulary_quick(input_dir),
    };
    save(output_file, &doc_id, &vocabulary)
}

fn vocabulary_greedy(input_dir: ReadDir) -> (Vec<String>, Vec<VocabularyEntry>) {
    let mut doc_id: Vec<String> = Vec::new();
    let mut vocabulary: Vec<VocabularyEntry> = Vec::new();
    for entry in input_dir {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                continue;
            }
            let contents = match fs::read_to_string(entry.path()) {
                Ok(_contents) => _contents,
                Err(_) => {
                    println!(
                        "File with path {:?} contains invalid Unicode and will be skipped",
                        entry.path()
                    );
                    continue;
                }
            };

            let contents: Vec<String> = contents
                .split(|c: char| !c.is_alphabetic())
                .map(|s| s.to_lowercase())
                .collect();

            doc_id.push(
                entry
                    .path()
                    .to_str()
                    .expect("File name contains invalid Unicode")
                    .to_owned(),
            );

            for word in contents {
                let word = word.trim();
                match vocabulary.iter().position(|r| r.word() == word) {
                    Some(p) => vocabulary[p].push_id(doc_id.len() - 1),
                    None => vocabulary.push(VocabularyEntry::width_doc_id(
                        word.to_owned(),
                        doc_id.len() - 1,
                    )),
                };
            }
        }
    }
    vocabulary.sort();
    (doc_id, vocabulary)
}

fn vocabulary_greedy_multithread(input_dir: ReadDir) -> (Vec<String>, Vec<VocabularyEntry>) {
    let doc_id: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let vocabulary: Arc<Mutex<Vec<VocabularyEntry>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for entry in input_dir {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                continue;
            }
            let doc_id = Arc::clone(&doc_id);
            let vocabulary = Arc::clone(&vocabulary);
            let handle = thread::spawn(move || {
                let mut doc_id = doc_id.lock().unwrap();
                let mut vocabulary = vocabulary.lock().unwrap();
                let contents = match fs::read_to_string(entry.path()) {
                    Ok(_contents) => _contents,
                    Err(_) => {
                        println!(
                            "File with path {:?} contains invalid Unicode and will be skipped",
                            entry.path()
                        );
                        return;
                    }
                };
                let contents: Vec<String> = contents
                    .split(|c: char| !c.is_alphabetic())
                    .map(|s| s.to_lowercase())
                    .collect();

                doc_id.push(
                    entry
                        .path()
                        .to_str()
                        .expect("File name contains invalid Unicode")
                        .to_owned(),
                );
                for word in contents {
                    let word = word.trim();
                    match vocabulary.iter().position(|r| r.word() == word) {
                        Some(p) => vocabulary[p].push_id(doc_id.len() - 1),
                        None => vocabulary.push(VocabularyEntry::width_doc_id(
                            word.to_owned(),
                            doc_id.len() - 1,
                        )),
                    };
                }
            });
            handles.push(handle);
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let doc_id = Arc::try_unwrap(doc_id).expect("Lock still has multiple owners");
    let doc_id = doc_id.into_inner().expect("Mutex cannot be locked");

    let vocabulary = Arc::try_unwrap(vocabulary).expect("Lock still has multiple owners");
    let mut vocabulary = vocabulary.into_inner().expect("Mutex cannot be locked");
    vocabulary.sort();

    (doc_id, vocabulary)
}

fn vocabulary_quick(input_dir: ReadDir) -> (Vec<String>, Vec<VocabularyEntry>) {
    let mut doc_id: Vec<String> = Vec::new();
    let mut vocabulary: Vec<VocabularyEntry> = Vec::new();
    for entry in input_dir {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                continue;
            }
            let contents = match fs::read_to_string(entry.path()) {
                Ok(_contents) => _contents,
                Err(_) => {
                    println!(
                        "File with path {:?} contains invalid Unicode and will be skipped",
                        entry.path()
                    );
                    continue;
                }
            };
            doc_id.push(
                entry
                    .path()
                    .to_str()
                    .expect("File name contains invalid Unicode")
                    .to_owned(),
            );
            let mut contents: Vec<String> = contents
                .split(|c: char| !c.is_alphabetic())
                .map(|s| s.to_lowercase())
                .collect();
                
            contents.sort();
            contents.dedup();
            for word in contents {
                if word.len() < 1 {
                    continue;
                }
                vocabulary.push(VocabularyEntry::width_doc_id(
                    word.to_owned(),
                    doc_id.len() - 1,
                ));
            }
        }
    }
    join(&mut vocabulary);
    (doc_id, vocabulary)
}

fn join(vocabulary: &mut Vec<VocabularyEntry>) {
    vocabulary.sort();
    let set: Vec<_> = vocabulary.drain(..).collect();
    for entry in set {
        if let Some(last) = vocabulary.last_mut() {
            if last == &entry {
                last.push_ids(entry.doc_id());
                continue;
            }
        }
        vocabulary.push(entry);
    }
}

fn save(output_file: &str, doc_id: &Vec<String>, vocabulary: &Vec<VocabularyEntry>) -> Statistics {
    let mut result = String::new();
    result.push_str(&format!("{}\n", doc_id.len()));
    for i in 0..doc_id.len() {
        result.push_str(&format!("{}: {} \n", i, doc_id[i]));
    }
    for entry in vocabulary {
        result.push_str(&format!("{}: {:?}\n", entry.word(), entry.doc_id()))
    }
    match fs::write(output_file, &result) {
        Ok(_) => {
            println!("Vocabulary was successfully saved!");
            Statistics {
                files: doc_id.len(),
                entries: vocabulary.len(),
                vocabulary_size: fs::metadata(output_file).unwrap().len(),
            }
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => println!("Output file not found"),
                ErrorKind::PermissionDenied => {
                    println!("Not enough permissions to write to output file")
                }
                _ => println!("Cannot write to the output file"),
            }
            process::exit(0);
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
    const OUTPUT_FILE: &str = "vocabulary.txt";

    fn bench_greedy() {
        let start = Instant::now();
        vocabulary(INPUT_DIR, OUTPUT_FILE, Algorithm::Greedy);
        let duration = start.elapsed();
        println!("greedy indexing completed in {:?}", duration);
    }

    fn test_greedy_multithread() {
        let start = Instant::now();
        vocabulary(INPUT_DIR, OUTPUT_FILE, Algorithm::GreedyMultithread);
        let duration = start.elapsed();
        println!("greedy indexing completed in {:?}", duration);
    }

    #[test]
    fn bench_quick() {
        let start = Instant::now();
        vocabulary(INPUT_DIR, OUTPUT_FILE, Algorithm::Quick);
        let duration = start.elapsed();
        println!("greedy indexing completed in {:?}", duration);
    }
}
