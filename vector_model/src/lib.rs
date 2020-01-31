pub mod clustering;
pub mod document_vector;
pub mod query_processor;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // #[test]
    // pub fn file_extend_works() {
    //     let mut index = DocumentIndex::new();
    //     let dir = "/Users/michaelfilonenko/Downloads/test_collection";
    //     index.extend_from_dir(dir);
    //     println!("{:?}", index);
    // }
    #[test]
    pub fn document_vector_works() {
        let path_1 = Path::new("/Users/michaelfilonenko/projects/vector_model/first.txt");
        let path_2 = Path::new("/Users/michaelfilonenko/projects/vector_model/second.txt");
        let vector_1 = document_vector::DocumentVector::from_path(path_1).unwrap();
        let vector_2 = document_vector::DocumentVector::from_path(path_2).unwrap();
        println!("{:?}", vector_1);
        println!("{:?}", vector_2);
        println!("len {}", vector_1.len());
        println!("len {}", vector_2.len());
        println!("mul {}", vector_1.mul(&vector_2));
        println!("mul {}", vector_2.mul(&vector_1));
        println!("sim {}", vector_2.sim(&vector_1));
    }
}
