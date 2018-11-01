/*
cargo run -p std_misc --bin process_wait
*/

use std::process::Command;

fn main() {
    println!("main start");
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
