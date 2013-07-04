#[link(name = "git2",
       vers = "0.1-pre",
       url = "https://github.com/kimhyunkang/git2-rs")];

#[comment = "libgit2 binding for Rust"];
#[license = "MIT"];

#[crate_type = "lib"];

pub mod ext;
pub mod repository;
pub mod reference;
pub mod git_index;
pub mod tree;
pub mod blob;
pub mod commit;
pub mod signature;
pub mod oid;

condition! {
    git_error: (~str, super::GitError) -> ();
}

pub unsafe fn raise() {
    git_error::cond.raise(last_error())
}

pub unsafe fn last_error() -> (~str, GitError) {
    let err = ext::giterr_last();
    let message = std::str::raw::from_c_str((*err).message);
    let klass = (*err).klass;
    (message, klass)
}

/** Error classes */
pub enum GitError {
    GITERR_NOMEMORY,
    GITERR_OS,
    GITERR_INVALID,
    GITERR_REFERENCE,
    GITERR_ZLIB,
    GITERR_REPOSITORY,
    GITERR_CONFIG,
    GITERR_REGEX,
    GITERR_ODB,
    GITERR_INDEX,
    GITERR_OBJECT,
    GITERR_NET,
    GITERR_TAG,
    GITERR_TREE,
    GITERR_INDEXER,
    GITERR_SSL,
    GITERR_SUBMODULE,
    GITERR_THREAD,
    GITERR_STASH,
    GITERR_CHECKOUT,
    GITERR_FETCHHEAD,
    GITERR_MERGE,
}

pub struct Repository {
    priv repo: *ext::git_repository,
}

pub struct Reference<'self> {
    priv c_ref: *ext::git_reference,
    priv owner: &'self Repository,
}

pub struct GitIndex<'self> {
    priv index: *ext::git_index,
    priv owner: &'self Repository,
}

pub struct Tree<'self> {
    priv tree: *ext::git_tree,
    priv owner: &'self Repository,
}

pub struct TreeEntry {
    priv tree_entry: *ext::git_tree_entry,
    priv owned: bool,
}

pub struct TreeBuilder {
    priv bld: *ext::git_treebuilder,
}

impl TreeBuilder {
    /// Create a new tree builder.
    /// The tree builder can be used to create or modify trees in memory and
    /// write them as tree objects to the database.
    /// The tree builder will start with no entries and will have to be filled manually.
    pub fn new() -> TreeBuilder
    {
        let mut bld:*ext::git_treebuilder = std::ptr::null();
        unsafe {
            if ext::git_treebuilder_create(&mut bld, std::ptr::null()) == 0 {
                TreeBuilder { bld: bld }
            } else {
                fail!(~"failed to create treebuilder")
            }
        }
    }

    /// Create a new tree builder.
    /// The tree builder will be initialized with the entries of the given tree.
    pub fn from_tree(tree: &Tree) -> TreeBuilder
    {
        let mut bld:*ext::git_treebuilder = std::ptr::null();
        unsafe {
            if ext::git_treebuilder_create(&mut bld, tree.tree) == 0 {
                TreeBuilder { bld: bld }
            } else {
                fail!(~"failed to create treebuilder")
            }
        }
    }
}

pub struct Blob<'self> {
    priv blob: *ext::git_blob,
    priv owner: &'self Repository,
}

pub struct Commit<'self> {
    priv commit: *ext::git_commit,
    priv owner: &'self Repository,
}

pub struct Time {
    pub time: i64,      /* time in seconds from epoch */
    pub offset: int,    /* timezone offset, in minutes */
}

#[deriving(Eq)]
pub struct Signature {
    pub name: ~str,
    pub email: ~str,
    pub when: Time,
}

pub struct OID {
    pub id: [std::libc::c_char, ..20],
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

impl Status {
    /// set every flags to false
    pub fn new() -> Status {
        Status {
            index_new: false,
            index_modified: false,
            index_deleted: false,
            index_renamed: false,
            index_typechange: false,

            wt_new: false,
            wt_modified: false,
            wt_deleted: false,
            wt_typechange: false,

            ignored: false,
        }
    }
}

/// Valid modes for index and tree entries.
pub enum FileMode {
	GIT_FILEMODE_NEW					= 0x0000,   // 0000000
	GIT_FILEMODE_TREE					= 0x4000,   // 0040000
	GIT_FILEMODE_BLOB					= 0x81a4,   // 0100644
	GIT_FILEMODE_BLOB_EXECUTABLE		= 0x81ed,   // 0100755
	GIT_FILEMODE_LINK					= 0xa000,   // 0120000
	GIT_FILEMODE_COMMIT					= 0xe000,   // 0160000
}

/// Basic type (loose or packed) of any Git object.
pub enum OType {
	GIT_OBJ_ANY = -2,		// Object can be any of the following
	GIT_OBJ_BAD = -1,		// Object is invalid.
	GIT_OBJ__EXT1 = 0,		// Reserved for future use.
	GIT_OBJ_COMMIT = 1,		// A commit object.
	GIT_OBJ_TREE = 2,		// A tree (directory listing) object.
	GIT_OBJ_BLOB = 3,		// A file revision object.
	GIT_OBJ_TAG = 4,		// An annotated tag object.
	GIT_OBJ__EXT2 = 5,		// Reserved for future use.
	GIT_OBJ_OFS_DELTA = 6,  // A delta, base is given by an offset.
	GIT_OBJ_REF_DELTA = 7,  // A delta, base is given by object id.
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
