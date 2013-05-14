extern mod git2;

use core::path::Path;

#[test]
fn test_open() {
    let path = Path(".");
    let open = git2::repository::open(&path);
    assert_eq!(open.is_ok(), true);
}
