extern mod git2;

#[test]
fn test_open() {
    let open:Result<@git2::Repository, git2::GitError> = git2::repository::open(".");
    assert_eq!(open.is_ok(), true);
}

#[test]
fn test_head() {
    let head_result = {
        let open:Result<@git2::Repository, git2::GitError> = git2::repository::open(".");
        assert_eq!(open.is_ok(), true);
        open.get().head()
    };
    assert_eq!(head_result.is_ok(), true);
}
