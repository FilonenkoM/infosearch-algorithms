#[macro_use]
extern crate maplit;

pub mod spimi;
mod token;
mod utils;
mod vocabulary;

#[cfg(test)]
mod tests {
    use self::token::Token;
    use self::vocabulary::Vocabulary;
    use super::*;

    #[test]
    fn tokens_works() {
        println!("Token creation");
        let token = Token::new(String::new(), 0);
        println!("{}", token);
        println!("{:?}", token);
    }
    #[test]
    fn token_getters_setters() {
        println!("Basic functionality of the token testing");
        let mut token = Token::new(String::new(), 42);
        println!("{}", token.term());
        println!("{}", token.doc_id());
        *token.doc_id_mut() += 10;
        println!("{}", token);
        println!("{:?}", token);
        token.term_set(String::from("the new string!1"));
        token.doc_id_set(42);
        println!("{}", token);
        println!("{:?}", token);
    }

    #[test]
    fn vocabulary_works() {
        println!("Basic vocabulary functions");
        let mut vocabulary = Vocabulary::new();
        println!("{}", vocabulary);
        println!("{:?}", vocabulary);
        vocabulary.add_token(Token::new(String::from("abc"), 0));
        vocabulary.add_token(Token::new(String::from("abc"), 1));
        println!("{}", vocabulary);
        println!("{:?}", vocabulary);
    }

    #[test]
    fn parse_entry_works() {
        println!("Testing parsing of the string into parts");
        let input = "abc: 0, 1,";
        println!("{:?}", spimi::get_string_components(input));
    }
    #[test]
    fn merge_pair_works() {
        println!("Testing of the merge pair of files");
        spimi::merge_pair("first.txt", "second.txt", "result.txt");
    }
    #[test]
    fn read_lines_works() {
        println!("Testing reading file line by line");
        for line in utils::read_lines("first.txt") {
            println!("{}", line.unwrap());
        }
    }

    #[test]
    fn merge_files_works() {
        spimi::merge_files(5);
    }

    #[test]
    pub fn vocabulary_save_works() {
        let mut v = Vocabulary::new();
        v.add_token(Token::new(String::from("first"), 0));
        v.add_token(Token::new(String::from("second"), 1));
        v.add_token(Token::new(String::from("first"), 1));
        v.save("result.txt");
    }

    #[test]
    pub fn spimi_works() {
        spimi::spimi("files");
    }
}
