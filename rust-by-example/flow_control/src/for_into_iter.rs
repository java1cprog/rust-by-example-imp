// cargo run -p flow_control --bin for_into_iter

fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];
    let iterator: std::vec::IntoIter<&str> = names.into_iter();
    for name in iterator {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("{}", &names[0]);

    let v = vec!["a".to_string(), "b".to_string()];
    for s in v.into_iter() {
        // s has type String, not &String
        println!("{}", s);
    }
   // println!("{}", v[0]);
}