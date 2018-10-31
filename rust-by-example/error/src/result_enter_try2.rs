/*
cargo run -p error --bin result_enter_try2
*/
use std::io;
use std::fs::File;
use std::io::prelude::*;

enum MyError {
    FileWriteError
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> MyError {
        MyError::FileWriteError
    }
}

// The preferred method of quick returning Errors
fn write_to_file_question() -> Result<(), MyError> {
    let mut file = File::create("my_best_friends1.txt")?;
    file.write_all(b"This is a list of my best friends.")?;
    Ok(())
}

// The previous method of quick returning Errors
fn write_to_file_using_try() -> Result<(), MyError> {
    let mut file = try!(File::create("my_best_friends2.txt"));
    try!(file.write_all(b"This is a list of my best friends."));
    Ok(())
}

// This is equivalent to:
fn write_to_file_using_match() -> Result<(), MyError> {
    let mut file = try!(File::create("my_best_friends3.txt"));
    match file.write_all(b"This is a list of my best friends.") {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    }
    Ok(())
}

fn main() {
    let _ = write_to_file_question();
    let _ = write_to_file_using_try();
    let _ = write_to_file_using_match();
}