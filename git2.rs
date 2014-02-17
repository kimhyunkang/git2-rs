#[comment = "libgit2 binding for Rust"];
#[license = "MIT"];

#[crate_type = "lib"];

use std::{ptr, vec, str, cast};
use std::libc::{c_uint, c_char, c_int, c_void};

pub use reference::Reference;
pub use commit::Commit;
pub use tree::Tree;
pub use git_index::GitIndex;
pub use blob::Blob;

pub mod ext;
pub mod reference;
pub mod git_index;
pub mod tree;
pub mod blob;
pub mod commit;
pub mod signature;
pub mod oid;
pub mod diff;

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

pub enum WalkMode {
    WalkSkip = 1,
    WalkPass = 0,
    WalkStop = -1,
}

pub enum DiffDelta {
    GIT_DELTA_UNMODIFIED = 0, // no changes
    GIT_DELTA_ADDED = 1,      // entry does not exist in old version
    GIT_DELTA_DELETED = 2,    // entry does not exist in new version
    GIT_DELTA_MODIFIED = 3,   // entry content changed between old and new
    GIT_DELTA_RENAMED = 4,    // entry was renamed between old and new
    GIT_DELTA_COPIED = 5,     // entry was copied from another old entry
    GIT_DELTA_IGNORED = 6,    // entry is ignored item in workdir
    GIT_DELTA_UNTRACKED = 7,  // entry is untracked item in workdir
    GIT_DELTA_TYPECHANGE = 8, // type of entry changed between old and new
}

pub struct DiffList {
    priv difflist: *ext::git_diff_list,
}

pub struct Time {
    time: i64,      /* time in seconds from epoch */
    offset: int,    /* timezone offset, in minutes */
}

#[deriving(Eq)]
pub struct Signature {
    name: ~str,
    email: ~str,
    when: Time,
}

pub struct OID {
    id: [std::libc::c_char, ..20],
}

fn with_opt_c_str<T>(s: Option<&str>, f: |*c_char| -> T) -> T
{
    match s {
        None => f(ptr::null()),
        Some(r) => r.with_c_str(f)
    }
}

/// Status flags for a single file.
///
/// A combination of these values will be returned to indicate the status of a file.
/// Status compares the working directory, the index, and the current HEAD of the repository.
/// The `index` set of flags represents the status of file in the index relative to the HEAD,
/// and the `wt` set of flags represent the status of the file in the working directory
/// relative to the index.
pub struct Status {
    index_new: bool,
    index_modified: bool,
    index_deleted: bool,
    index_renamed: bool,
    index_typechange: bool,

    wt_new: bool,
    wt_modified: bool,
    wt_deleted: bool,
    wt_typechange: bool,

    ignored: bool,
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
    GIT_FILEMODE_NEW                    = 0x0000,   // 0000000
    GIT_FILEMODE_TREE                   = 0x4000,   // 0040000
    GIT_FILEMODE_BLOB                   = 0x81a4,   // 0100644
    GIT_FILEMODE_BLOB_EXECUTABLE        = 0x81ed,   // 0100755
    GIT_FILEMODE_LINK                   = 0xa000,   // 0120000
    GIT_FILEMODE_COMMIT                 = 0xe000,   // 0160000
}

/// Basic type (loose or packed) of any Git object.
#[repr(int)]
pub enum OType {
    GIT_OBJ_ANY = -2,       // Object can be any of the following
    GIT_OBJ_BAD = -1,       // Object is invalid.
    GIT_OBJ__EXT1 = 0,      // Reserved for future use.
    GIT_OBJ_COMMIT = 1,     // A commit object.
    GIT_OBJ_TREE = 2,       // A tree (directory listing) object.
    GIT_OBJ_BLOB = 3,       // A file revision object.
    GIT_OBJ_TAG = 4,        // An annotated tag object.
    GIT_OBJ__EXT2 = 5,      // Reserved for future use.
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

static PATH_BUF_SZ: uint = 1024u;

pub struct Repository {
    priv repo: *ext::git_repository,
}

impl Repository {
    /// Open a git repository.
    ///
    /// The 'path' argument must point to either a git repository folder, or an existing work dir.
    ///
    /// The method will automatically detect if 'path' is a normal
    /// or bare repository or raise bad_repo if 'path' is neither.
    pub fn open(path: &str) -> Result<Repository, (~str, GitError)>
    {
        unsafe {
            let mut ptr_to_repo: *ext::git_repository = ptr::null();
            path.with_c_str({ |c_path|
                if ext::git_repository_open(&mut ptr_to_repo, c_path) == 0 {
                    Ok( Repository::new(ptr_to_repo) )
                } else {
                    Err( last_error() )
                }
            })
        }
    }

    /// Creates a new Git repository in the given folder.
    /// if is_bare is true, a Git repository without a working directory is
    /// created at the pointed path. If false, provided path will be
    /// considered as the working directory into which the .git directory
    /// will be created.
    pub fn init(path: &str, is_bare: bool) -> Result<Repository, (~str, GitError)>
    {
        unsafe {
            let mut ptr_to_repo: *ext::git_repository = ptr::null();
            path.with_c_str(|c_path| {
                if ext::git_repository_init(&mut ptr_to_repo, c_path, is_bare as c_uint) == 0 {
                    Ok( Repository::new(ptr_to_repo) )
                } else {
                    Err( last_error() )
                }
            })
        }
    }

    /// Look for a git repository and copy its path in the given buffer.
    /// The lookup start from base_path and walk across parent directories
    /// if nothing has been found. The lookup ends when the first repository
    /// is found, or when reaching a directory referenced in ceiling_dirs
    /// or when the filesystem changes (in case across_fs is true).
    ///
    /// The method will automatically detect if the repository is bare
    /// (if there is a repository).
    ///
    /// ceiling_dirs: A GIT_PATH_LIST_SEPARATOR separated list of
    /// absolute symbolic link free paths. The lookup will stop when any
    /// of this paths is reached. Note that the lookup always performs on
    /// start_path no matter start_path appears in ceiling_dirs ceiling_dirs
    /// might be empty string
    pub fn discover(start_path: &str, across_fs: bool, ceiling_dirs: &str) -> Option<~str>
    {
        unsafe {
            let mut buf = vec::from_elem(PATH_BUF_SZ, 0u8 as c_char);
            let c_path = buf.as_mut_ptr();
            start_path.with_c_str(|c_start_path| {
                ceiling_dirs.with_c_str(|c_ceiling_dirs| {
                    let result = ext::git_repository_discover(c_path, PATH_BUF_SZ as u64,
                                            c_start_path, across_fs as c_int, c_ceiling_dirs);
                    let imm_path = c_path as *c_char;
                    if result == 0 {
                        Some( str::raw::from_c_str(imm_path) )
                    } else {
                        None
                    }
                })
            })
        }
    }

    /// Clone a remote repository, and checkout the branch pointed to by the remote
    /// this function do not receive options for now
    pub fn clone(url: &str, local_path: &str) -> Result<Repository, (~str, GitError)> {
        unsafe {
            let mut ptr_to_repo: *ext::git_repository = ptr::null();
            url.with_c_str(|c_url| {
                local_path.with_c_str(|c_path| {
                    if ext::git_clone(&mut ptr_to_repo, c_url, c_path, ptr::null()) == 0 {
                        Ok( Repository::new(ptr_to_repo) )
                    } else {
                        Err( last_error() )
                    }
                })
            })
        }
    }

    fn new(repo: *ext::git_repository) -> Repository {
        Repository { repo: repo }
    }

    /// Get the path of this repository
    ///
    /// This is the path of the `.git` folder for normal repositories,
    /// or of the repository itself for bare repositories.
    pub fn path(&self) -> ~str {
        unsafe {
            let c_path = ext::git_repository_path(self.repo);
            str::raw::from_c_str(c_path)
        }
    }

    /// Get the path of the working directory for this repository
    ///
    /// If the repository is bare, this function will always return None.
    pub fn workdir(&self) -> Option<~str> {
        unsafe {
            let c_path = ext::git_repository_workdir(self.repo);
            if ptr::is_null(c_path) {
                None
            } else {
                Some(str::raw::from_c_str(c_path))
            }
        }
    }

    /// Retrieve and resolve the reference pointed at by HEAD.
    pub fn head<'r>(&'r self) -> Option<~Reference<'r>> {
        unsafe {
            let mut ptr_to_ref: *ext::git_reference = ptr::null();

            match ext::git_repository_head(&mut ptr_to_ref, self.repo) {
                0 => Some( ~Reference::new(ptr_to_ref, self) ),
                ext::GIT_EORPHANEDHEAD => None,
                ext::GIT_ENOTFOUND => None,
                _ => {
                    git_error::cond.raise(last_error());
                    None
                },
            }
        }
    }

    /// Lookup a reference by name in a repository.
    /// The name will be checked for validity.
    pub fn lookup<'r>(&'r self, name: &str) -> Option<~Reference<'r>> {
        unsafe {
            let mut ptr_to_ref: *ext::git_reference = ptr::null();

            name.with_c_str(|c_name| {
                if(ext::git_reference_lookup(&mut ptr_to_ref, self.repo, c_name) == 0) {
                    Some( ~Reference::new(ptr_to_ref, self) )
                } else {
                    None
                }
            })
        }
    }

    /// Lookup a branch by its name in a repository.
    ///
    /// The generated reference must be freed by the user.
    ///
    /// The branch name will be checked for validity.
    /// See `git_tag_create()` for rules about valid names.
    ///
    /// Returns None if the branch name is invalid, or the branch is not found
    ///
    /// remote: True if you want to consider remote branch,
    ///     or false if you want to consider local branch
    pub fn lookup_branch<'r>(&'r self, branch_name: &str, remote: bool) -> Option<~Reference<'r>>
    {
        let mut ptr: *ext::git_reference = ptr::null();
        let branch_type = if remote { ext::GIT_BRANCH_REMOTE } else { ext::GIT_BRANCH_LOCAL };
        branch_name.with_c_str(|c_name| {
            unsafe {
                let res = ext::git_branch_lookup(&mut ptr, self.repo, c_name, branch_type);
                match res {
                    0 => Some( ~Reference::new(ptr, self) ),
                    ext::GIT_ENOTFOUND => None,
                    ext::GIT_EINVALIDSPEC => None,
                    _ => {
                        git_error::cond.raise(last_error());
                        None
                    },
                }
            }
        })
    }

    /// Lookup a commit object from repository
    pub fn lookup_commit<'r>(&'r self, id: &OID) -> Option<~Commit<'r>> {
        unsafe {
            let mut commit: *ext::git_commit = ptr::null();
            if ext::git_commit_lookup(&mut commit, self.repo, id) == 0 {
                Some( ~Commit::new(commit, self) )
            } else {
                None
            }
        }
    }

    /// Lookup a tree object from repository
    pub fn lookup_tree<'r>(&'r self, id: &OID) -> Option<~Tree<'r>> {
        unsafe {
            let mut tree: *ext::git_tree = ptr::null();
            if ext::git_tree_lookup(&mut tree, self.repo, id) == 0 {
                Some( ~Tree::new(tree, self) )
            } else {
                None
            }
        }
    }

    /// Updates files in the index and the working tree to match the content of
    /// the commit pointed at by HEAD.
    /// This function does not accept options for now
    ///
    /// returns true when successful, false if HEAD points to an non-existing branch
    /// raise on other errors
    pub fn checkout_head(&self) -> bool {
        unsafe {
            match ext::git_checkout_head(self.repo, ptr::null()) {
                0 => true,
                ext::GIT_EORPHANEDHEAD => false,
                _ => {
                    git_error::cond.raise(last_error());
                    false
                }
            }
        }
    }

    /// Get the Index file for this repository.
    ///
    /// If a custom index has not been set, the default
    /// index for the repository will be returned (the one
    /// located in `.git/index`).
    pub fn index<'r>(&'r self) -> Result<~GitIndex<'r>, (~str, GitError)> {
        unsafe {
            let mut ptr_to_ref: *ext::git_index = ptr::null();

            if ext::git_repository_index(&mut ptr_to_ref, self.repo) == 0 {
                Ok( ~GitIndex::new(ptr_to_ref, self) )
            } else {
                Err( last_error() )
            }
        }
    }

    /// Check if a repository is empty
    pub fn is_empty(&self) -> bool {
        unsafe {
            let res = ext::git_repository_is_empty(self.repo);
            if res < 0 {
                git_error::cond.raise(last_error());
                false
            } else {
                res != 0
            }
        }
    }

    /// Check if a repository is bare
    pub fn is_bare(&self) -> bool {
        unsafe {
            ext::git_repository_is_bare(self.repo) != 0
        }
    }

    /// Create a new branch pointing at a target commit
    ///
    /// A new direct reference will be created pointing to
    /// this target commit. If `force` is true and a reference
    /// already exists with the given name, it'll be replaced.
    ///
    /// The returned reference must be freed by the user.
    ///
    /// The branch name will be checked for validity.
    /// See `git_tag_create()` for rules about valid names.
    pub fn branch_create<'r>(&'r mut self, branch_name: &str, target: &Commit, force: bool)
        -> Option<~Reference<'r>>
    {
        let mut ptr: *ext::git_reference = ptr::null();
        let flag = force as c_int;
        unsafe {
            branch_name.with_c_str(|c_name| {
                let res = ext::git_branch_create(&mut ptr, self.repo, c_name, target.commit, flag);
                match res {
                    0 => Some( ~Reference::new(ptr, self) ),
                    ext::GIT_EINVALIDSPEC => None,
                    _ => {
                        git_error::cond.raise(last_error());
                        None
                    },
                }
            })
        }
    }

    /// Return the name of the reference supporting the remote tracking branch,
    /// given the name of a local branch reference.
    pub fn upstream_name(&self, canonical_branch_name: &str) -> Option<~str>
    {
        let mut buf: [c_char, ..1024] = [0, ..1024];
        canonical_branch_name.with_c_str(|c_name| {
            let v = buf.as_mut_ptr();
            unsafe {
                let res = ext::git_branch_upstream_name(v, 1024, self.repo, c_name);
                if res >= 0 {
                    let ptr: *u8 = cast::transmute(v);
                    Some( str::raw::from_buf_len(ptr, res as uint) )
                } else if res == ext::GIT_ENOTFOUND {
                    None
                } else {
                    git_error::cond.raise(last_error());
                    None
                }
            }
        })
    }

    /// Return the name of remote that the remote tracking branch belongs to.
    /// returns Err(GIT_ENOTFOUND) when no remote matching remote was found,
    /// returns Err(GIT_EAMBIGUOUS) when the branch maps to several remotes,
    pub fn git_branch_remote_name(&self, canonical_branch_name: &str)
        -> Result<~str, (~str, GitError)>
    {
        let mut buf: [c_char, ..1024] = [0, ..1024];
        canonical_branch_name.with_c_str(|c_name| {
            let v = buf.as_mut_ptr();
            unsafe {
                let res = ext::git_branch_remote_name(v, 1024, self.repo, c_name);
                if res >= 0 {
                    let ptr: *u8 = cast::transmute(v);
                    Ok( str::raw::from_buf_len(ptr, res as uint) )
                } else {
                    Err( last_error() )
                }
            }
        })
    }

    /// Lookup a blob object from a repository.
    pub fn blob_lookup<'r>(&'r self, id: &OID) -> Option<~Blob<'r>>
    {
        let mut ptr: *ext::git_blob = ptr::null();
        unsafe {
            if ext::git_blob_lookup(&mut ptr, self.repo, id) == 0 {
                Some( ~Blob::new(ptr, self) )
            } else {
                None
            }
        }
    }

    /// Read a file from the working folder of a repository
    /// and write it to the Object Database as a loose blob
    pub fn blob_create_fromworkdir<'r>(&'r self, relative_path: &str)
        -> Result<~Blob<'r>, (~str, GitError)>
    {
        let mut oid = OID { id: [0, ..20] };
        let mut ptr: *ext::git_blob = ptr::null();
        relative_path.with_c_str(|c_path| {
            unsafe {
                if ext::git_blob_create_fromworkdir(&mut oid, self.repo, c_path) == 0 {
                    if ext::git_blob_lookup(&mut ptr, self.repo, &oid) != 0 {
                        fail!(~"blob lookup failure");
                    }
                    Ok( ~Blob::new(ptr, self) )
                } else {
                    Err( last_error() )
                }
            }
        })
    }

    /// Read a file from the filesystem and write its content
    /// to the Object Database as a loose blob
    pub fn blob_create_fromdisk<'r>(&'r self, relative_path: &str)
        -> Result<~Blob<'r>, (~str, GitError)>
    {
        let mut oid = OID { id: [0, ..20] };
        let mut ptr: *ext::git_blob = ptr::null();
        relative_path.with_c_str(|c_path| {
            unsafe {
                if ext::git_blob_create_fromdisk(&mut oid, self.repo, c_path) == 0 {
                    if ext::git_blob_lookup(&mut ptr, self.repo, &oid) != 0 {
                        fail!(~"blob lookup failure");
                    }
                    Ok( ~Blob::new(ptr, self) )
                } else {
                    Err( last_error() )
                }
            }
        })
    }

    /// Write an in-memory buffer to the ODB as a blob
    pub fn blob_create_frombuffer<'r>(&'r self, buffer: &[u8])
        -> Result<~Blob<'r>, (~str, GitError)>
    {
        let mut oid = OID { id: [0, ..20] };
        let v = buffer.as_ptr();
        let len = buffer.len() as u64;
        unsafe {
            let buf:*c_void = cast::transmute(v);
            if ext::git_blob_create_frombuffer(&mut oid, self.repo, buf, len) == 0 {
                let mut ptr: *ext::git_blob = ptr::null();
                if ext::git_blob_lookup(&mut ptr, self.repo, &oid) != 0 {
                    fail!(~"blob lookup failure");
                }
                Ok( ~Blob::new(ptr, self) )
            } else {
                Err( last_error() )
            }
        }
    }

    /// Create new commit in the repository from a list of Commit pointers
    ///
    /// Returns the created commit. The commit will be written to the Object Database and
    ///  the given reference will be updated to point to it
    ///
    /// id: Pointer in which to store the OID of the newly created commit
    ///
    /// update_ref: If not None, name of the reference that
    ///  will be updated to point to this commit. If the reference
    ///  is not direct, it will be resolved to a direct reference.
    ///  Use "HEAD" to update the HEAD of the current branch and
    ///  make it point to this commit. If the reference doesn't
    ///  exist yet, it will be created.
    ///
    /// author: Signature with author and author time of commit
    ///
    /// committer: Signature with committer and commit time of commit
    ///
    /// message_encoding: The encoding for the message in the
    ///  commit, represented with a standard encoding name.
    ///  E.g. "UTF-8". If None, no encoding header is written and
    ///  UTF-8 is assumed.
    ///
    /// message: Full message for this commit
    ///
    /// tree: An instance of a Tree object that will
    ///  be used as the tree for the commit. This tree object must
    ///  also be owned by `self`
    ///
    /// parents: Vector of Commit objects that will be used as the parents for this commit.
    ///  All the given commits must be owned by `self`.
    pub fn commit<'r>(&'r self, update_ref: Option<&str>, author: &Signature,
            committer: &Signature, message_encoding: Option<&str>, message: &str, tree: &Tree,
            parents: &[~Commit<'r>]) -> OID
    {
        unsafe {
            let c_author = signature::to_c_sig(author);
            let c_committer = signature::to_c_sig(committer);
            let mut oid = OID { id: [0, .. 20] };
            let c_parents = parents.map(|p| { p.commit });
            let parent_ptr = c_parents.as_ptr();
            let len = c_parents.len() as c_int;
            let res =
            with_opt_c_str(update_ref, |ref_ptr| {
                with_opt_c_str(message_encoding, |enc_ptr| {
                    message.with_c_str(|msg_ptr| {
                        ext::git_commit_create(&mut oid, self.repo, ref_ptr,
                            &c_author, &c_committer, enc_ptr, msg_ptr, tree.tree, len, parent_ptr)
                    })
                })
            });

            if res != 0 {
                git_error::cond.raise(last_error());
            }
            oid
        }
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        unsafe {
            ext::git_repository_free(self.repo);
        }
    }
}
