mod inverted_index;
mod inverted_index_entry;
mod trigram_index;
mod trigram_index_entry;
mod utils;
mod vocabulary;
mod permutation_index;
mod permutation_index_entry;

use self::inverted_index_entry::InvertedIndexEntry;
use self::vocabulary::Vocabulary;
use std::fs;
use std::fs::DirEntry;
use std::fs::ReadDir;
use std::io::ErrorKind;
use std::process;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    const INPUT_DIR: &str = "files";
    const OUTPUT_DIR: &str = "vocabulary";

    #[test]
    fn bench() {
        let start = Instant::now();
        let mut v = Vocabulary::new();
        v.fill_from_file(INPUT_DIR);
        v.fill_trigram_index();

        v.save(OUTPUT_DIR);
        let duration = start.elapsed();
        println!("indexing completed in {:?}", duration);
    }
}
