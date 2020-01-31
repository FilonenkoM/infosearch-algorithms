use std::io;
use std::time::Instant;
use vector_model::clustering::Cluster;
use vector_model::query_processor::QueryProcessor;

use std::path::Path;

fn main() {
    let path = Path::new("/Users/michaelfilonenko/Downloads/gutenberg_output_flat 2");
    let now = Instant::now();
    let processor = QueryProcessor::from_clusters(Cluster::from_dir(&path).clusteing());
    println!("Processed in {:?}", now.elapsed());

    println!("Indexing finished, I am ready to your questions");
    loop {
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let now = Instant::now();
        processor.process_query(&query);
        println!("Processed in {:?}", now.elapsed());
        let now = Instant::now();
        processor.process_query_clipping_clusters(&query);
        println!("Processed in {:?}", now.elapsed());
    }
}
