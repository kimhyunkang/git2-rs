pub use super::*;

condition! {
    bad_repo: (~str, super::GitError) -> @mut super::Repository;
}

condition! {
    bad_path: (~str, super::GitError) -> ~str;
}

condition! {
    bad_ref: (~str, super::GitError) -> ~super::Reference;
}

condition! {
    bad_index: (~str, super::GitError) -> ~super::GitIndex;
}

condition! {
    bad_tree: (~str, super::GitError) -> ~super::Tree;
}

condition! {
    bad_treeentry: (~str, super::GitError) -> ~super::TreeEntry;
}

condition! {
    bad_treebuilder: (~str, super::GitError) -> super::TreeBuilder;
}

condition! {
    bad_oid: (~str, super::GitError) -> super::OID;
}

condition! {
    bad_commit: (~str, super::GitError) -> ~super::Commit;
}

condition! {
    check_fail: (~str, super::GitError) -> bool;
}

condition! {
    checkout_fail: (~str, super::GitError) -> ();
}

condition! {
    index_fail: (~str, super::GitError) -> ();
}

condition! {
    commit_fail: (~str, super::GitError) -> ();
}

condition! {
    iter_fail: (~str, super::GitError) -> ();
}
