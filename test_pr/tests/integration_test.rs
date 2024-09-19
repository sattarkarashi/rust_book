use test_pr;
mod common;

#[test]
fn it_adds_two(){
    assert_eq!(4,test_pr::adds_two(2));
    common::set_up();

    // for integration tests to run, all the unit test first should pass. If not, the integration tests will not run
}