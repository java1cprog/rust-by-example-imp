
// cargo run -p flow_control --bin while

fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("\tfizz");
        } else if n % 5 == 0 {
            println!("\t\tbuzz");
        } else {
            println!("\t\t\t{}", n);
        }

        // Increment counter
        n += 1;
    }
}