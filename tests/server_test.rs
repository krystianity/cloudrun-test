use crate::common::tear_down;

mod common;

#[test]
fn test_server() {
    let mut cloudrun_server = common::setup();

    assert_eq!(1, 1);

    tear_down(&mut cloudrun_server);
}
