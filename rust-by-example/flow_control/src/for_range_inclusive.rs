// cargo run -p flow_control --bin for_range_inclusive

fn main() {
    let range = 1..=100;
    //let iterator = range.iter_into();
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in range {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("\tfizz");
        } else if n % 5 == 0 {
            println!("\t\tbuzz");
        } else {
            println!("\t\t\t{}", n);
        }
    }
}