use testing_in_rust;

mod common;

#[test]
fn it_adds_two() {
    common::some_common_function();

    let result = testing_in_rust::add_two(2);
    assert_eq!(result, 4);
}