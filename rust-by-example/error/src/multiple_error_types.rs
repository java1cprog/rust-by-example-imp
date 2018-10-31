/*
 cargo run -p error --bin multiple_error_types
*/
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn main() {

    let numbers = vec!["42", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    if false
    {
        let empty = vec![];
        println!("The first doubled is {}", double_first(empty));
        // Error 1: the input vector is empty

    }

    {
        let strings = vec!["tofu", "93", "18"];
        println!("The first doubled is {}", double_first(strings));
        // Error 2: the element doesn't parse to a number
    }

}

