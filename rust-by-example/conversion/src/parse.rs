// cargo run -p conversion --bin parse

fn main() {
    let parsed:u32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<u32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {}", sum};

//    let parsed_i32 = "5i32".parse().unwrap();
//    println!{"{}", parsed_i32};

}