use std::time::Instant;
use zone_search::zone_data_reader;
use zone_search::zone_data_reader::BookData;
use zone_search::zone_index_generator::ZoneIndex;

// use zone_search::zone_index_generator::ZoneIndex;

fn main() {
    let path = "/Users/michaelfilonenko/Downloads/gutenberg_output_flat";
    println!("Starting indexing the collection");
    let inst = Instant::now();
    let data = BookData::parse_collection(path);
    let index = ZoneIndex::from_book_data(&data);
    println!("({:?}) I am listen to your questions", inst.elapsed());
    loop {
        let mut query = String::new();
        std::io::stdin().read_line(&mut query).unwrap();
        if let Some(ids) = index.select(&query) {
            for title in zone_data_reader::get_titles(&data, &ids) {
                println!("{}", title.trim());
            }
        }
        else { println!("No results found"); }
    }
}
