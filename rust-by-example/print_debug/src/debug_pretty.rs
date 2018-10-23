// cargo run -p print_debug --bin debu_pretty
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name: &str = "Peter";
    let age: u8 = 27;
    let peter: Person = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}