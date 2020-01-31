use large_index::spimi;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let path = "/Users/michaelfilonenko/projects/large_index/files";
    spimi::spimi(path);
    println!("Processed in {:?}", now.elapsed());
}

