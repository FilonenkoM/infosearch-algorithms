use std::fs;
use std::fs::DirEntry;
use std::io::ErrorKind;
use std::fs::ReadDir;
use std::process;
use std::io::Lines;
use std::io::{BufRead, BufReader, Result};
use std::fs::File;
use std::fs::OpenOptions;

pub fn read_file(file: &DirEntry) -> Option<String> {
    match fs::read_to_string(file.path()) {
        Ok(result) => Some(result),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!(
                    "File {} not found. Maybe, it was deleted during the process",
                    file.path().display()
                ),
                ErrorKind::PermissionDenied => println!(
                    "Not enough permissions to read the {}",
                    file.path().display()
                ),
                ErrorKind::InvalidData => println!(
                    "The file {} contains invalid Unicode",
                    file.path().display()
                ),
                _ => println!("Cannot read the file {}", file.path().display()),
            }
            None
        }
    }
}
pub fn read_dir(dir: &str) -> ReadDir {
    match fs::read_dir(dir) {
        Ok(result) => result,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => println!("Input directory does not exist"),
                ErrorKind::PermissionDenied => {
                    println!("Not enough permissions to read from this directory")
                }
                _ => println!("Cannot read files from the given directory"),
            }
            process::exit(0);
        }
    }
}
pub fn write(output: &str, contents: &str) -> bool {
    match fs::write(output, contents) {
        Ok(_) => true,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("Incorrect path"),
                ErrorKind::PermissionDenied => {
                    println!("Not enough permissions to write to this file")
                }
                _ => println!("Can not write to this file"),
            }
            false
        }
    }
}

pub fn read_lines(file: &str) -> Lines<BufReader<File>> {
    BufReader::new(match File::open(file) {
        Ok(_file) => _file,
        Err(err) => {
            println!("{} {}", err, file);
            panic!()
        }
    }).lines()
}

pub fn file_write(file: &str) -> File {
    File::create(file);
    match OpenOptions::new().write(true).append(true).open(file) {
        Ok(_file) => _file,
        Err(err) => {
            eprintln!("{}", err);
            panic!();
        }
    }
}