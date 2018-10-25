//  cargo run -p primitives --bin literals
fn main() {

    let integer = 1; println!("{}", integer);
    let float = 1.2; println!("{}", float);
    let character = 'a'; println!("{}", character);
    let string = "abc"; println!("{}", string);
    let boolean = true; println!("{}", boolean);
    let unit_type = (); println!("{:?}", unit_type);

    let _int_16 = 0x0;
    let _int_8 = 0o0;
    let _int_binary = 0b0;

    // using underscore
    let value1 = 1_000;
    let value_1_1 = 1000;
    assert_eq!(value1,value_1_1);
    let value2 = 0.000_001;
    let value2_1 =  0.000001;
    assert_eq!(value2,value2_1);

    use std::mem;
    println!("size of integer {}", mem::size_of_val(&integer));
    println!("size of float {}", mem::size_of_val(&float));
    println!("size of character {}", mem::size_of_val(&character));
    println!("size of string {}", mem::size_of_val(string));
    println!("size of boolean {}", mem::size_of_val(&boolean));
    println!("size of unit type {}", mem::size_of_val(&()));
}