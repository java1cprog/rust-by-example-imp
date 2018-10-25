// cargo run -p conversion --bin string

use std::string::ToString;
//use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {}", self.radius)
    }
}
//impl fmt::Display for Circle {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self.radius)
//    }
//}
fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    //println!("{}", circle);
    println!("{:?}", circle);
}

