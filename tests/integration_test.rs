mod common;

use rust_book::api;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, api::add_two(2));
}