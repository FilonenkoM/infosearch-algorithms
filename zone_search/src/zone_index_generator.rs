use crate::zone_data_reader::BookData;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::iter::FromIterator;

// pub struct PostingList {
//     title: Vec<usize>,
//     author: Vec<usize>,
//     text: Vec<usize>,
// }

// impl PostingList {
//     pub fn new() -> PostingList {
//         PostingList {
//             title: Vec::new(),
//             author: Vec::new(),
//             text: Vec::new(),
//         }
//     }
//     pub fn title(&self) -> &Vec<usize> {
//         &self.title
//     }
//     pub fn author(&self) -> &Vec<usize> {
//         &self.author
//     }
//     pub fn text(&self) -> &Vec<usize> {
//         &self.text
//     }
//     pub fn title_mut(&mut self) -> &mut Vec<usize> {
//         &mut self.title
//     }
//     pub fn author_mut(&mut self) -> &mut Vec<usize> {
//         &mut self.author
//     }
//     pub fn text_mut(&mut self) -> &mut Vec<usize> {
//         &mut self.text
//     }
// }

// pub struct ZoneIndex {
//     map: BTreeMap<String, PostingList>,
// }

// impl ZoneIndex {
//     pub fn new() -> ZoneIndex {
//         ZoneIndex {
//             map: BTreeMap::new(),
//         }
//     }
//     pub fn extend_from_collection(&mut self, data: &Vec<BookData>) {
//         for (i, entry) in data.iter().enumerate() {
//             for word in entry.author().split(|c: char| !c.is_alphabetic()) {
//                 match self.map.get_mut(word) {
//                     Some(list) => {
//                         if !list.author().contains(&i) {
//                             list.author_mut().push(i);
//                         }
//                     }
//                     None => {
//                         let mut pl = PostingList::new();
//                         pl.author_mut().push(i);
//                         self.map.insert(word.to_owned(), pl);
//                     }
//                 }
//             }
//             for word in entry.title().split(|c: char| !c.is_alphabetic()) {
//                 match self.map.get_mut(word) {
//                     Some(list) => {
//                         if !list.title().contains(&i) {
//                             list.title_mut().push(i);
//                         }
//                     }
//                     None => {
//                         let mut pl = PostingList::new();
//                         pl.title_mut().push(i);
//                         self.map.insert(word.to_owned(), pl);
//                     }
//                 }
//             }
//             for word in entry.text().split(|c: char| !c.is_alphabetic()) {
//                 match self.map.get_mut(word) {
//                     Some(list) => {
//                         if !list.text().contains(&i) {
//                             list.text_mut().push(i);
//                         }
//                     }
//                     None => {
//                         let mut pl = PostingList::new();
//                         pl.text_mut().push(i);
//                         self.map.insert(word.to_owned(), pl);
//                     }
//                 }
//             }
//         }
//     }
//     pub fn save(&self, file: &str) {
//         let mut result = String::new();
//         for (key, value) in &self.map {
//             write!(result, "{} ", key).unwrap();
//             for doc in value.author() {
//                 write!(result, "{} ", doc).unwrap();
//             }
//             write!(result, "# ").unwrap();
//             for doc in value.title() {
//                 write!(result, "{} ", doc).unwrap();
//             }
//             write!(result, "# ").unwrap();
//             for doc in value.text() {
//                 write!(result, "{} ", doc).unwrap();
//             }
//             write!(result, "\n").unwrap();
//         }
//         if let Err(e) = fs::write(file, result) {
//             println!("{}", e);
//         }
//     }
// }
pub const WEIGHTS: [f32; 7] = [
    0.3,
    0.2,
    0.05,
    0.2,
    0.1,
    0.1,
    0.05,
];

type Term = String;
type DocID = usize;
type FieldID = i8;
type PostingList = BTreeMap<DocID, Vec<FieldID>>;
pub struct ZoneIndex {
    map: BTreeMap<Term, PostingList>,
}
impl ZoneIndex {
    pub fn new() -> Self {
        ZoneIndex {
            map: BTreeMap::new(),
        }
    }
    pub fn map(&self) -> &BTreeMap<Term, PostingList> {
        &self.map
    }
    pub fn from_book_data(data: &Vec<BookData>) -> Self {
        let mut index = Self::new();
        for (i, entry) in data.iter().enumerate() {
            for (field_id, value) in entry.fields() {
                let value = value.to_lowercase();
                for word in value.split(|c: char| !c.is_alphanumeric()) {
                    // if word.contains("Lincoln") {
                    //     println!("{}", word);
                    //     println!("{}", field_id);
                    // }
                    if !word.is_empty() {
                        match index.map.get_mut(word) {
                            Some(list) => {
                                match list.get_mut(&i) {
                                    Some(field_ids) => {
                                        if !field_ids.contains(field_id) {
                                            field_ids.push(*field_id);
                                        }
                                    }
                                    None => {
                                        list.insert(i, vec![*field_id]);
                                    }
                                };
                            }
                            None => {
                                let mut list = PostingList::new();
                                list.insert(i, vec![*field_id]);
                                index.map.insert(word.to_owned(), list);
                            }
                        }
                    }
                }
            }
        }
        index
    }
    pub fn save(&self, file: &Path) {
        let mut res = String::new();
        for (term, list) in &self.map {
            write!(res, "{} ", term).unwrap();
            for (doc_id, field_ids) in list {
                write!(res, "{}.", doc_id).unwrap();
                for field_id in field_ids {
                    write!(res, "{} ", field_id).unwrap();
                }
            }
            write!(res, "\n").unwrap();
        }
        fs::write(file, &res).expect("Unable to save to the output file");
    }

   pub fn select(&self, query: &str) -> Option<Vec<usize>> {
        #[derive(Debug, PartialEq, Copy, Clone)]
        enum Operator {
            And,
            Or,
            Not,
            LeftParen,
            RightParen,
        }
        #[derive(Debug)]
        enum Token {
            Operator(Operator),
            Word(String),
        }
        fn normalize_query(query: &str) -> String {
            let mut normalized = String::new();
            for c in query.chars() {
                if c == '(' || c == ')' {
                    normalized.push(' ');
                }
                normalized.push(c); 
                if c == ')' || c == '(' {
                    normalized.push(' ');
                }
            }
            if cfg!(debug_assertions) {
                println!("normalized {}", normalized);
            }
            normalized
        }
        fn shunting_yard(query: &mut Vec<Token>) -> Option<Vec<Token>> {
            let mut operator_stack: Vec<Operator> = Vec::new();
            let mut output_queue: Vec<Token> = Vec::new();
            match query[0] {
                Token::Operator(Operator::Not) => query.insert(0, Token::Word("*".to_owned())),
                _ => (),
            }
            for token in query {
                match token {
                    Token::Word(_word) => output_queue.push(Token::Word(_word.clone())),
                    Token::Operator(operator) => match operator {
                        Operator::And | Operator::Or | Operator::Not => {
                            while let Some(o) = operator_stack.pop() {
                                if o != Operator::LeftParen {
                                    output_queue.push(Token::Operator(o));
                                } else {
                                    break;
                                }
                            }
                            operator_stack.push(operator.clone());
                        }
                        Operator::LeftParen => {
                            operator_stack.push(operator.clone());
                        }
                        Operator::RightParen => {
                            while let Some(o) = operator_stack.pop() {
                                if o != Operator::LeftParen {
                                    output_queue.push(Token::Operator(o));
                                } else {
                                    break;
                                }
                            }
                            if let Some(o) = operator_stack.pop() {
                                if o != Operator::LeftParen {
                                    return None;
                                }
                            }
                        }
                    },
                }
            }
            for operator in operator_stack {
                match operator {
                    Operator::LeftParen | Operator::RightParen => return None,
                    _ => output_queue.push(Token::Operator(operator)),
                }
            }
            Some(output_queue)
        }
        fn to_token(query: &str) -> Token {
            match query.trim().as_ref() {
                "(" => Token::Operator(Operator::LeftParen),
                ")" => Token::Operator(Operator::RightParen),
                "AND" => Token::Operator(Operator::And),
                "OR" => Token::Operator(Operator::Or),
                "NOT" => Token::Operator(Operator::Not),
                _ => Token::Word(query.to_owned()),
            }
        }
        // self.map.get(word).or_else(BTreeSet::new())
        fn process_stack(index: &ZoneIndex, stack: &Vec<Token>) -> Vec<usize> {
            #[derive(Debug)]
            enum PToken {
                Operand(PostingList),
                Operator(Operator),
            }
            let stack: Vec<PToken> = stack
                .iter()
                .map(|t| match t {
                    Token::Operator(o) => PToken::Operator(o.clone()),
                    Token::Word(w) => match w.as_ref() {
                        "*" => {
                            let set = PostingList::new();
                            // for i in 0..index.docs.len() {
                            //     set.insert(i);
                            // }
                            PToken::Operand(set)
                        }
                        _ => PToken::Operand(
                            index
                                .map
                                .get(w)
                                .unwrap_or(&PostingList::new())
                                .iter()
                                .map(|list| {
                                    let mut v = Vec::new();
                                    for n in list.1 {
                                        v.push(*n);
                                    }
                                    (*list.0, v)
                                })
                                .collect(),
                        ),
                    },
                })
                .collect();
            let mut output_stack: Vec<PToken> = Vec::new();
            for token in stack {
                match token {
                    PToken::Operand(o) => output_stack.push(PToken::Operand(o)),
                    PToken::Operator(o) => {
                        if let Some(PToken::Operand(op1)) = output_stack.pop() {
                            if let Some(PToken::Operand(op2)) = output_stack.pop() {
                                let mut op1 = op1;
                                let keys1: BTreeSet<usize> = op1.keys().map(|u| *u).collect();
                                let keys2: BTreeSet<usize> = op2.keys().map(|u| *u).collect();
                                let mut result = PostingList::new();
                                match o {
                                    Operator::And => {
                                        for key in keys1.intersection(&keys2) {
                                            let mut list: Vec<i8> = Vec::new();
                                            if let Some(list1) = op1.get_mut(key) {
                                                list.append(list1);
                                            }
                                            if let Some(list2) = op1.get_mut(key) {
                                                list.append(list2);
                                            }
                                            list.sort();
                                            list.dedup();
                                            result.insert(*key, list);
                                        }
                                    }
                                    Operator::Or => {
                                        println!("or");
                                        for key in keys1.union(&keys2) {
                                            let mut list: Vec<i8> = Vec::new();
                                            if let Some(list1) = op1.get_mut(key) {
                                                list.append(list1);
                                            }
                                            if let Some(list2) = op1.get_mut(key) {
                                                list.append(list2);
                                            }
                                            list.sort();
                                            list.dedup();
                                            result.insert(*key, list);
                                        }
                                    }
                                    Operator::Not => {
                                        for key in keys2.difference(&keys1) {
                                            let mut list: Vec<i8> = Vec::new();
                                            if let Some(list1) = op1.get_mut(key) {
                                                list.append(list1);
                                            }
                                            if let Some(list2) = op1.get_mut(key) {
                                                list.append(list2);
                                            }
                                            list.sort();
                                            list.dedup();
                                            result.insert(*key, list);
                                        }
                                    }
                                    _ => {
                                        return Vec::new();
                                    }
                                };
                                output_stack.push(PToken::Operand(result));
                            } else {
                                return Vec::new();
                            }
                        } else {
                            return Vec::new();
                        }
                    }
                }
            }
            let mut weighted_map: BTreeMap<usize, f32> = BTreeMap::new();
            if let Some(PToken::Operand(list)) = output_stack.pop() {
                for (doc_id, field_ids) in list {
                    let mut weight: f32 = 0.0;
                    for field_id in field_ids {
                        weight += WEIGHTS[field_id as usize];
                    }
                    weighted_map.insert(doc_id, weight);
                }
            }
            let mut answer = Vec::from_iter(weighted_map);
            answer.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
            let answer: Vec<usize> = answer.iter().map(|(doc_id, _)| *doc_id).collect();
            answer
        }
        if cfg!(debug_assertions) {
            println!("query: {}", query);
        }
        if query.is_empty() {
            return None;
        }
        let normalized = normalize_query(query);
        let mut query_list: Vec<Token> = normalized
            .split_whitespace()
            .map(|s: &str| to_token(s))
            .collect();
        if cfg!(debug_assertions) {
            println!("query list: {:?}", query_list);
        }
        if let Some(stack) = shunting_yard(&mut query_list) {
            if cfg!(debug_assertions) {
                println!("stack: {:?}", stack);
            }
            println!("{:?}", process_stack(self, &stack));
            return Some(process_stack(self, &stack));
            // return process_stack(self, &stack);
        }
        None
    }
}
