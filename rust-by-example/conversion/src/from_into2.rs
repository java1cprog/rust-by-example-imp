// cargo run -p conversion --bin from_into2
use std::convert::From;
use core::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    println!("My number is {}", num);
}
