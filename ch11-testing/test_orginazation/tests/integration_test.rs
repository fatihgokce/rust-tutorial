use test_orginazation;
mod common;
#[test]
fn integration_adds_two() {
    //just run integration test
    //cargo test --test integration_test
    common::setup();
    assert_eq!(4, test_orginazation::add_two(2));
}
//To avoid having common appear in the test output,
// instead of creating tests/common.rs,
// we’ll create tests/common/mod.rs.