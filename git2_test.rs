extern mod git2;

#[test]
fn test_open() {
    let open = git2::repository::open(".");
    assert_eq!(open.is_ok(), true);
}
