#[link(name = "git2",
       vers = "0.1-pre",
       url = "https://github.com/kimhyunkang/git2-rs")];

#[comment = "libgit2 binding for Rust"];
#[license = "MIT"];

#[crate_type = "lib"];

extern mod std;

pub mod ext;
pub mod conditions;
pub mod repository;
pub mod reference;
pub mod index;
pub mod tree;
pub mod commit;
pub mod signature;
pub mod oid;

pub struct Repository {
    pub repo: *ext::git_repository,
}

pub struct Reference {
    pub c_ref: *ext::git_reference,
    pub repo_ptr: @Repository,
}

pub struct GitIndex {
    pub index: *ext::git_index,
    pub owner: @Repository,
}

pub struct Tree {
    pub tree: *ext::git_tree,
    pub owner: @Repository,
}

pub struct Commit {
    pub commit: *ext::git_commit,
    pub owner: @Repository,
}

pub struct Signature {
    pub name: ~str,
    pub email: ~str,
    pub when: std::time::Tm,
}

pub struct OID {
    pub id: [core::libc::c_char, ..20],
}

/// Status flags for a single file.
///
/// A combination of these values will be returned to indicate the status of a file.
/// Status compares the working directory, the index, and the current HEAD of the repository.
/// The `index` set of flags represents the status of file in the index relative to the HEAD,
/// and the `wt` set of flags represent the status of the file in the working directory
/// relative to the index.
pub struct Status {
    pub index_new: bool,
    pub index_modified: bool,
    pub index_deleted: bool,
    pub index_renamed: bool,
    pub index_typechange: bool,

    pub wt_new: bool,
    pub wt_modified: bool,
    pub wt_deleted: bool,
    pub wt_typechange: bool,

    pub ignored: bool,
}
// FIXME: there should be better ways to do this...
// if you call this library in multiple tasks,
// this function must be called before calling any other functions in library
pub fn threads_init() {
    unsafe {
        ext::git_threads_init();
    }
}

// if you call this library in multiple tasks,
// this function must be called before shutting down the library
pub fn threads_shutdown() {
    unsafe {
        ext::git_threads_shutdown();
    }
}
