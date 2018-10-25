// cargo run -p flow_control --bin match

fn main() {
    let number = 20;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    for i in 1..=number {
        match i {
            // Match a single value
            1 => print!("[One! {}] ",i),
            // Match several values
            2 | 3 | 5 | 7 | 11 => print!("[This is a prime {}] ",i),
            // Match an inclusive range
            13...19 => print!("[A teen {}] ",i),
            // Handle the rest of cases
            _ => print!("[Ain't special {}] ",i),
        }
    }
    println!();
    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
      //  false => -10,
        true => 1,
       // true => 2,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
