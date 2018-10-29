// cargo build -p cfg

// NOTE THAT: LIB DO NOT CREATE - APP WAS CREATED
// This crate is a library
#![crate_type = "lib"]
// The library is named "cfg"
#![crate_name = "cfg"]

pub fn public_function() {
    println!("called my_new_crate's `public_function()`");
}

fn private_function() {
    println!("called my_new_crate's `private_function()`");
}

pub fn indirect_access() {
    print!("called my_new_crate's `indirect_access()`, that\n> ");

    private_function();
}
fn main(){}