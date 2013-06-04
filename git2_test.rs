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
