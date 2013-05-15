extern mod git2;

#[test]
fn test_open() {
    let open:Result<@git2::Repository, git2::GitError> = git2::repository::open(".");
    assert_eq!(open.is_ok(), true);
}
