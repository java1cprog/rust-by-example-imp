// cargo run -p flow_control --bin for_iter

use std::slice::Iter;

fn main() {
    let names = vec!["Bob", "Frank", "Ferris", "Anatolij", "Konstantin", "Nikolaj"];
    let iterator: Iter<&str> = names.iter();
    for name in iterator {
        match name {
            &"Bob" => println!("There is a rustacean Bob among us!"),
            &"Frank" => println!("There is a rustacean Frank among us!"),
            &"Ferris" => println!("There is a rustacean Ferris among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let slice = &[1, 2, 3];

    // Then, we iterate over it:
    for element in slice.iter() {
        print!(", {}", element);
    }
}