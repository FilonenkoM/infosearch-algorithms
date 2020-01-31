pub mod zone_data_reader;
pub mod zone_index_generator;

#[cfg(test)]
mod tests {
    use self::zone_data_reader;
    use self::zone_data_reader::BookData;
    use self::zone_index_generator::ZoneIndex;
    use super::*;
    // use self::zone_index_generator::ZoneIndex;
    use std::path::Path;

    // #[test]
    // fn read_collection_works() {
    //     let path = "/Users/michaelfilonenko/Downloads/gutenberg_output_flat";
    //     let instant = Instant::now();
    //     zone_data_reader::read_gutenberg_collection(path);
    //     println!("time {:?}", instant.elapsed());
    // }

    // #[test]
    // fn generator_works() {
    //     let path = "/Users/michaelfilonenko/Downloads/gutenberg_output_flat";
    //     let instant = Instant::now();
    //     let collection = zone_data_reader::read_gutenberg_collection(path);
    //     let mut index = ZoneIndex::new();
    //     index.extend_from_collection(&collection);
    //     index.save("index.txt");
    //     println!("{:?}", instant.elapsed());
    // }

    #[test]
    fn new_parser_works() {
        let path = Path::new("/Users/michaelfilonenko/Downloads/gutenberg_output_flat/25787.txt");
        let book_data = BookData::from_path(&path);
        println!("{:?}", book_data);
    }

    #[test]
    fn collection_works() {
        let path = "/Users/michaelfilonenko/Downloads/gutenberg_output_flat";
        let bd = BookData::parse_collection(path);
        println!("{:?}", bd);
    }

    #[test]
    fn zone_index_works() {
        let path = "/Users/michaelfilonenko/Downloads/gutenberg_output_flat";
        let data = BookData::parse_collection(path);
        let index = ZoneIndex::from_book_data(&data);
        // let output_path = Path::new("result.txt");
        // index.save(&output_path);
        println!("I am listen to your questions");
        loop {
            let mut query = String::new();
            std::io::stdin().read_line(&mut query).unwrap();
            println!("{:?}", index.select(&query));
        }
    }
}
