extern mod git2;
use core::path::Path;

#[test]
fn test_head() {
    let repo = git2::repository::open("fixture");
    let head_id = repo.head().resolve();
    assert_eq!(head_id.to_str(), ~"21002f5d3f411fe990e13604273a51cd598a4a51")
}

#[test]
fn test_path() {
    let repo = git2::repository::open("fixture");

    // submodule path
    let expected = os::make_absolute(&Path("../.git/modules/test/fixture")).normalize();
    let given = os::make_absolute(&Path(repo.path())).normalize();
    assert_eq!(given, expected)
}

#[test]
fn test_lookup() {
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
fn test_empty() {
    let repo = git2::repository::open("fixture");
    assert_eq!(repo.is_empty(), false)
}

#[test]
fn test_bare() {
    let repo = git2::repository::open("fixture");
    assert_eq!(repo.is_bare(), false)
}

#[test]
fn test_oid() {
    let oid_str = ~"93d8ad7e3f5a300d2b4e18e9f31247a95e5cc37f";
    let oid = git2::oid::from_str(oid_str).get();
    assert_eq!(oid.to_str(), oid_str)
}
