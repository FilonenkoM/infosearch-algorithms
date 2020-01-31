use crate::token::Token;
use crate::utils;
use crate::vocabulary::Vocabulary;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Lines;
use std::io::BufReader;

const ESCAPE_CHARS: [char; 25] = [
    ' ', ',', '.', ':', ';', '|', '/', '(', ')', '{', '}', '[', ']', '"', '\'', '*', '\n', '\t',
    '&', '#', '-', '$', '#', '!', '?',
];
// the files in this function has standard names, such as block_1.txt .. block_n-1.txt

pub fn spimi(input_dir: &str) {
    let mut block_id = 0;
    let mut block = 0;
    let mut vocabulary = Vocabulary::new();
    for (index, file) in utils::read_dir(input_dir).enumerate() {
        let contents = match utils::read_file(&file.unwrap()) {
            Some(_contents) => _contents,
            None => continue,
        };
        let tokens = contents
            .split(|c: char| ESCAPE_CHARS.contains(&c) || c.is_numeric())
            .map(|s: &str| Token::new(String::from(s.to_lowercase()), index));
        for token in tokens {
            if !token.term().is_empty() {
                vocabulary.add_token(token);
            }
        }
        if index % 500 == 0 {
            vocabulary.save(&format!("block_{}.txt", block_id));
            vocabulary = Vocabulary::new();
            block_id += 1;
        }
        block += 1;
    }
    vocabulary.save(&format!("block_{}.txt", block_id));
    merge_files(block_id + 1);
}

pub fn merge_files(amount: usize) -> String {
    // two temporary files for merge sort, that will be switched during the merge
    let output_pair = ["output_0.txt", "output_1.txt"];
    let mut current = 0;
    if amount == 1 {
        return String::from("block_0.txt");
    }
    if amount == 2 {
        merge_pair("block_0.txt", "block_1.txt", output_pair[current]);
        return String::from(output_pair[1]);
    }
    merge_pair("block_0.txt", "block_1.txt", output_pair[current]);
    for i in 2..amount {
        merge_pair(
            output_pair[i % 2],
            &format!("block_{}.txt", i),
            output_pair[(i % 2 + 1) % 2],
        );
        current = (i % 2 + 1) % 2;
    }
    for i in 0..amount {
        fs::remove_file(format!("block_{}.txt", i)).expect("Unable to remove the file");
    }
    fs::remove_file(output_pair[(current + 1) % 2]).expect("Unable to remove file");
    String::from(output_pair[current])
}

pub fn merge_pair(first: &str, second: &str, target: &str) {
    struct State {
        has_next: bool,
        next_line_needed: bool,
        current_line: String,
        current_term: String,
        current_doc: String,
        stream: Lines<BufReader<File>>,
    }
    impl State {
        pub fn new(stream: Lines<BufReader<File>>) -> State {
            State {
                has_next: true,
                next_line_needed: true,
                current_term: String::from(""),
                current_doc: String::from(""),
                current_line: String::from(""),
                stream,
            }
        }
    }
    let mut output_stream = utils::file_write(target);
    let mut first_state = State::new(utils::read_lines(first));
    let mut second_state = State::new(utils::read_lines(second));
    while first_state.has_next || second_state.has_next {
        if first_state.next_line_needed {
            if let Some(line) = first_state.stream.next() {
                first_state.current_line = match line {
                    Ok(_line) => _line,
                    Err(err) => panic!(err),
                };
            } else {
                first_state.has_next = false;
            }
            first_state.next_line_needed = false;
        }
        if second_state.next_line_needed {
            if let Some(line) = second_state.stream.next() {
                second_state.current_line = match line {
                    Ok(_line) => _line,
                    Err(err) => panic!(err),
                };
            } else {
                second_state.has_next = false;
            }
            second_state.next_line_needed = false;
        }
        if first_state.has_next && second_state.has_next {
            let components = get_string_components(&first_state.current_line);
            first_state.current_term = components.0;
            first_state.current_doc = components.1;

            let components = get_string_components(&second_state.current_line);
            second_state.current_term = components.0;
            second_state.current_doc = components.1;

            if first_state.current_term < second_state.current_term {
                writeln!(output_stream, "{}", first_state.current_line.trim()).expect("Unable to write");
                first_state.next_line_needed = true;
            } else if first_state.current_term > second_state.current_term {
                writeln!(output_stream, "{}", second_state.current_line.trim()).expect("Unable to write");
                second_state.next_line_needed = true;
            } else {
                writeln!(
                    output_stream,
                    "{} {}",
                    first_state.current_line, second_state.current_doc
                ).expect("Unable to write");
                first_state.next_line_needed = true;
                second_state.next_line_needed = true;
            }
        } else if first_state.has_next {
            writeln!(output_stream, "{}", first_state.current_line.trim()).expect("Unable to write");
            first_state.next_line_needed = true;
        } else if second_state.has_next {
            writeln!(output_stream, "{}", second_state.current_line.trim()).expect("Unable to write");
            second_state.next_line_needed = true;
        }
    }
}

pub fn get_string_components(input: &str) -> (String, String) {
    let mut split = input.split(":");
    let term = match split.next() {
        Some(_term) => _term,
        None => panic!("The string {} does not contain a valid input", input),
    };
    let doc = match split.next() {
        Some(_doc) => _doc.trim(),
        None => panic!("The string {} does not contain a valid input", input),
    };
    (String::from(term), String::from(doc))
}
