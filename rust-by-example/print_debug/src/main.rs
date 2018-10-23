
// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`
// FIXME add `#[derive(Debug)]`
#[allow(dead_code)]
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
    //println!("{:?}", UnPrintable(0i32));
    println!("{:?}", DebugPrintable(0i32));

}
