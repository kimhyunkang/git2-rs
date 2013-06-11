extern mod git2;

#[test]
fn test_head() {
    let repo = git2::repository::open("fixture");
    let head_id = repo.head().resolve();
    assert_eq!(head_id.to_str(), ~"21002f5d3f411fe990e13604273a51cd598a4a51")
}

#[test]
fn test_oid() {
    let oid_str = ~"93d8ad7e3f5a300d2b4e18e9f31247a95e5cc37f";
    let oid = git2::oid::from_str(oid_str).get();
    assert_eq!(oid.to_str(), oid_str)
}
