// cargo run -p scope --bin borrow_freeze

#[allow(unused_assignments)]
fn main() {
    let mut mutable_integer = 7i32;

    {
        // Borrow `_mutable_integer`
        let _large_integer = &mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_large_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    mutable_integer = 3;

}