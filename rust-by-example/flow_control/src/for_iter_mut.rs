// cargo run -p flow_control --bin for_iter_mut

use std::slice::IterMut;

fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    let iterator: IterMut<&str> = names.iter_mut();

    for name in iterator {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => {
                println!("Hello {}", name);
            }
        }
    }


    let slice = &mut [1, 2, 3];

    // Then, we iterate over it and increment each element value:
    for element in slice.iter_mut() {
        *element += 1;
    }
    println!("{:?}", slice);
}