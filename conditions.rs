pub use ext::{git_error_t, git_oid};
pub use types::*;

condition! {
    bad_repo: (~str, super::git_error_t) -> @super::Repository;
}

condition! {
    bad_path: (~str, super::git_error_t) -> ~str;
}

condition! {
    bad_ref: (~str, super::git_error_t) -> ~super::Reference;
}

condition! {
    bad_index: (~str, super::git_error_t) -> ~super::GitIndex;
}

condition! {
    oid_fail: (~str, super::git_error_t) -> super::git_oid;
}

condition! {
    check_fail: (~str, super::git_error_t) -> bool;
}

condition! {
    checkout_fail: (~str, super::git_error_t) -> ();
}

condition! {
    index_fail: (~str, super::git_error_t) -> ();
}
