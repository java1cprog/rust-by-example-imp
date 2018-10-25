// cargo run -p flow_control --bin match_destructuring_destructure_tuple

fn main() {
    let pair:(i32,i32) = (0, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
println!("---------------------------------------------------");
    let vector = vec![(0, -2),(2, 0),(2, -2)];
    for pair in vector.iter() {
        match pair {
            // Destructure the second
            (0, y) => println!("First is `0` and `y` is `{:?}`", y),
            (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
            _      => println!("It doesn't matter what they are"),
            // `_` means don't bind the value to a variable
        }
    }
}