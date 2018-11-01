// extern crate we're testing, same as any other code would do.
extern crate integration_testing;

// importing common module.
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();
    assert_eq!(integration_testing::add(3, 2), 5);
}