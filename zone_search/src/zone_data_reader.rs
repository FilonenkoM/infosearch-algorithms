use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

// pub struct BookData {
//     title: String,
//     author: String,
//     text: String,
// }

// impl BookData {
//     pub fn title(&self) -> &String {
//         &self.title
//     }
//     pub fn author(&self) -> &String {
//         &self.author
//     }
//     pub fn text(&self) -> &String {
//         &self.text
//     }
// }

// pub fn read_gutenberg_collection(dir: &str) -> Vec<BookData> {
//     let mut data: Vec<BookData> = Vec::new();
//     if let Ok(dir) = fs::read_dir(dir) {
//         for entry in dir {
//             if let Ok(entry) = entry {
//                 if let Some(book) = read_gutenberg_book(&entry) {
//                     data.push(book);
//                 }
//             }
//         }
//     }
//     data
// }

// fn read_gutenberg_book(file: &DirEntry) -> Option<BookData> {
//     fn first_word_is(line: &str, word: &str) -> bool {
//         if let Some(_word) = line.split_whitespace().next() {
//             return _word.trim() == word;
//         }
//         return false;
//     }
//     fn find_start(lines: &mut Lines) -> Option<String> {
//         loop {
//             if let Some(line) = lines.next() {
//                 if first_word_is(line, "Title:") {
//                     return Some(line.to_owned());
//                 }
//             } else {
//                 return None;
//             }
//         }
//     }
//     fn parse_metadata(line: &str) -> Option<String> {
//         let mut split = line.split(|c: char| c == ':');
//         if let Some(_) = split.next() {
//             if let Some(word) = split.next() {
//                 return Some(word.trim().to_owned());
//             }
//         }
//         None
//     }
//     fn get_next_metadata(lines: &mut Lines) -> Option<String> {
//         match lines.next() {
//             Some(data_line) => match parse_metadata(data_line) {
//                 Some(_dat_line) => Some(_dat_line),
//                 _ => return None,
//             },
//             _ => return None,
//         }
//     }
//     let contents = match fs::read_to_string(file.path()) {
//         Ok(_contents) => _contents,
//         Err(_) => return None,
//     };
//     let mut lines = contents.lines();
//     let start = find_start(&mut lines);
//     let start = match start {
//         Some(_start) => _start,
//         None => return None,
//     };
//     let title = {
//         if let Some(title) = parse_metadata(&start) {
//             title
//         } else {
//             return None;
//         }
//     };
//     let author = {
//         match lines.next() {
//             Some(_) => match get_next_metadata(&mut lines) {
//                 Some(_author) => _author,
//                 None => return None,
//             },
//             _ => return None,
//         }
//     };
//     let mut text = String::new();
//     for line in lines {
//         write!(&mut text, "{}\n", line).unwrap();
//     }

//     let title = title.to_lowercase();
//     let author = author.to_lowercase();
//     let text = text.to_lowercase();
//     Some(BookData {
//         title,
//         author,
//         text,
//     })
// }
#[derive(Debug)]
pub struct BookData {
    fields: BTreeMap<i8, String>, // where first string is field and second string is text for this term
}
pub const ACCEPTIBLE_FIELDS: [&str; 7] = [
    "Title:",
    "Author:",
    "Text:",
    "Language:",
    "Release Date:",
    "Posting Date:",
    "Character set encoding:",
];

impl BookData {
    pub fn new() -> BookData {
        BookData {
            fields: BTreeMap::new(),
        }
    }

    pub fn from_path(entry: &Path) -> Option<BookData> {
        let mut book_data = Self::new();
        if let Ok(contents) = fs::read_to_string(entry) {
            let mut start_reached = false;
            let mut current_field: (i8, String) = (-1, String::new());
            let mut lines = contents.lines();
            while let Some(line) = lines.next() {
                if line.starts_with("*** START") || line.starts_with("***START") {
                    start_reached = true;
                    current_field.0 = 2;
                }
                let mut start_found = false;
                for i in 0..ACCEPTIBLE_FIELDS.len() {
                    let field = ACCEPTIBLE_FIELDS[i];
                    if line.starts_with(field) && !start_reached {
                        start_found = true;
                        if current_field.0 != -1 {
                            book_data.fields.insert(current_field.0 as i8, current_field.1.clone());
                        }
                        current_field.0 = i as i8;
                        current_field.1 = line[(field.len() + 1)..].to_owned();
                        break;
                    }
                }
                if !start_found {
                    writeln!(current_field.1, "{}", line).unwrap();
                }
            }
            if start_reached {
                book_data
                    .fields
                    .insert(current_field.0, current_field.1.clone());
                return Some(book_data);
            }
        }

        None
    }

    pub fn parse_collection(dir: &str) -> Vec<BookData> {
        let mut data: Vec<BookData> = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(book_data) = BookData::from_path(&entry.path()) {
                        data.push(book_data);
                    }
                }
            }
        }
        data
    }

    pub fn fields(&self) -> &BTreeMap<i8, String> {
        &self.fields
    }
    pub fn fields_mut(&mut self) -> &mut BTreeMap<i8, String> {
        &mut self.fields
    }
}

pub fn get_titles(data: &Vec<BookData>, ids: &Vec<usize>) -> Vec<String> {
    let mut titles = Vec::new();
    for id in ids {
        if id <= &data.len() {
            match data[*id].fields().get(&0) {
                Some(_title) => titles.push(_title.to_owned()),
                None => titles.push(format!("Book without a name: {}", id)),
            }
        }
    }
    titles
}