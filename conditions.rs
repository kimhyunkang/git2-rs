pub use ext::git_error_t;
pub use super::*;

condition! {
    bad_repo: (~str, super::git_error_t) -> @mut super::Repository;
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
    bad_tree: (~str, super::git_error_t) -> ~super::Tree;
}

condition! {
    bad_treeentry: (~str, super::git_error_t) -> ~super::TreeEntry;
}

condition! {
    bad_treebuilder: (~str, super::git_error_t) -> super::TreeBuilder;
}

condition! {
    bad_oid: (~str, super::git_error_t) -> super::OID;
}

condition! {
    bad_commit: (~str, super::git_error_t) -> ~super::Commit;
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

condition! {
    commit_fail: (~str, super::git_error_t) -> ();
}

condition! {
    iter_fail: (~str, super::git_error_t) -> ();
}
