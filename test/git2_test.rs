extern mod git2;

#[test]
fn repo_head() {
    let repo = git2::repository::open("fixture");
    let head_id = repo.head().resolve();
    assert_eq!(head_id.to_str(), ~"21002f5d3f411fe990e13604273a51cd598a4a51")
}

#[test]
fn repo_lookup() {
    let repo = git2::repository::open("fixture");

    match repo.lookup("refs/heads/master") {
        None => fail!(~"failed to lookup master ref"),
        Some(master_ref) => {
            let master_id = master_ref.resolve();
            assert_eq!(master_id.to_str(), ~"21002f5d3f411fe990e13604273a51cd598a4a51")
        },
    }
}

#[test]
fn repo_empty() {
    let repo = git2::repository::open("fixture");
    assert_eq!(repo.is_empty(), false)
}

#[test]
fn repo_bare() {
    let repo = git2::repository::open("fixture");
    assert_eq!(repo.is_bare(), false)
}

#[test]
fn repo_oid() {
    let oid_str = ~"93d8ad7e3f5a300d2b4e18e9f31247a95e5cc37f";
    let oid = git2::oid::from_str(oid_str);
    assert_eq!(oid.to_str(), oid_str)
}

#[test]
fn repo_lookup_commit() {
    let repo = git2::repository::open("fixture");
    let oid = git2::oid::from_str(&"21002f5d3f411fe990e13604273a51cd598a4a51");
    match repo.lookup_commit(&oid) {
        None => {
            fail!(~"commit does not exist")
        },
        Some(commit) => {
            assert!(commit.parents().is_empty(), ~"the first commit should have no parents");
            assert_eq!(commit.message(), ~"Create README.md");
        },
    }
}
