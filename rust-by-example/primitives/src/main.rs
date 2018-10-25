// cargo run -p primitives --bin primitives
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;println!("{}", logical);

    let a_float: f64 = 1.0; println!("{:.1}", a_float);println!("{:.*}", 2, a_float);  // Regular annotation
    let an_integer   = 5i32; println!("{}i32", an_integer); // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; println!("{:.1}f64", default_float); // `f64`
    let default_integer = 7; println!("{}i32", default_integer);   // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; println!("{}i64", inferred_type); // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    println!("{}i64", inferred_type);

    // A mutable variable's value can be changed.
    let mut mutable = 12;  println!("{}i32", mutable); // Mutable `i32`
    mutable = 21;
    println!("{}i32", mutable);

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
    println!("{}", mutable);

    println!("{:?}", [1, 2, 3]);
    println!("{:?}", (1, true));


}