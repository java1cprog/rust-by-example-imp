/*
cargo run -p unsafe_ --bin raw_pointers
*/

fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
