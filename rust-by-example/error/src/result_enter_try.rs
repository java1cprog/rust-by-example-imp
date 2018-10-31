/*
cargo run -p error --bin result_enter_try
*/
use std::num::ParseIntError;
//use std::io::prelude::*;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number:i32 = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

//    let first_number = first_number_str.prarse::<i32>()?;
//    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
