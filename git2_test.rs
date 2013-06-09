extern mod git2;

#[test]
fn test_open() {
    git2::repository::open(".");
}

#[test]
fn test_head() {
    let repo = git2::repository::open(".");
    repo.head();
}

#[test]
fn test_oid() {
    let oid_str = ~"93d8ad7e3f5a300d2b4e18e9f31247a95e5cc37f";
    let oid = git2::oid::from_str(oid_str).get();
    assert_eq!(oid.to_str(), oid_str)
}
