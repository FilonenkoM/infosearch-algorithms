use fuzzy_indices;
use rust_stemmers::{Algorithm, Stemmer};

fn main() {
    println!("Hello, world!");

    let en_stemmer = Stemmer::create(Algorithm::English);
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        let result = en_stemmer.stem(&line.trim());
        println!("{}", result);
    }
}

