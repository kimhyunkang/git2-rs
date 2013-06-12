use core::libc::{c_char, c_int, c_uint, c_void, size_t};
use ext;
use conditions;
use signature;
use super::*;

static PATH_BUF_SZ: uint = 1024u;

macro_rules! raise {
    ($cond_expr:expr) => ({
        let err = ext::giterr_last();
        let message = str::raw::from_c_str((*err).message);
        let klass = (*err).klass;
        $cond_expr.raise((message, klass))
    })
}

/// Open a git repository.
///
/// The 'path' argument must point to either a git repository folder, or an existing work dir.
///
/// The method will automatically detect if 'path' is a normal
/// or bare repository or raise bad_repo if 'path' is neither.
pub fn open(path: &str) -> @mut Repository
{
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(path) |c_path| {
            if ext::git_repository_open(&mut ptr_to_repo, c_path) == 0 {
                @mut Repository { repo: ptr_to_repo }
            } else {
                raise!(conditions::bad_repo::cond)
            }
        }
    }
}

/// Open a bare repository on the serverside.
///
/// This is a fast open for bare repositories that will come in handy
/// if you're e.g. hosting git repositories and need to access them
/// efficiently
pub fn open_bare(path: &str) -> @mut Repository
{
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(path) |c_path| {
            if ext::git_repository_open_bare(&mut ptr_to_repo, c_path) == 0 {
                @mut Repository { repo: ptr_to_repo }
            } else {
                raise!(conditions::bad_repo::cond)
            }
        }
    }
}

/// Creates a new Git repository in the given folder.
/// if is_bare is true, a Git repository without a working directory is
/// created at the pointed path. If false, provided path will be
/// considered as the working directory into which the .git directory
/// will be created.
pub fn init(path: &str, is_bare: bool) -> @mut Repository
{
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(path) |c_path| {
            if ext::git_repository_init(&mut ptr_to_repo, c_path, is_bare as c_uint) == 0 {
                @mut Repository { repo: ptr_to_repo }
            } else {
                raise!(conditions::bad_repo::cond)
            }
        }
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
pub fn discover(start_path: &str, across_fs: bool, ceiling_dirs: &str) -> ~str
{
    unsafe {
        let mut buf = vec::from_elem(PATH_BUF_SZ, 0u8 as c_char);
        do vec::as_mut_buf(buf) |c_path, sz| {
            do str::as_c_str(start_path) |c_start_path| {
                do str::as_c_str(ceiling_dirs) |c_ceiling_dirs| {
                    let result = ext::git_repository_discover(c_path, sz as size_t,
                                            c_start_path, across_fs as c_int, c_ceiling_dirs);
                    if result == 0 {
                        str::raw::from_buf(c_path as *u8)
                    } else {
                        raise!(conditions::bad_path::cond)
                    }
                }
            }
        }
    }
}

/// Clone a remote repository, and checkout the branch pointed to by the remote
/// this function do not receive options for now
pub fn clone(url: &str, local_path: &str) -> @mut Repository {
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(url) |c_url| {
            do str::as_c_str(local_path) |c_path| {
                if ext::git_clone(&mut ptr_to_repo, c_url, c_path, ptr::null()) == 0 {
                    @mut Repository { repo: ptr_to_repo }
                } else {
                    raise!(conditions::bad_repo::cond)
                }
            }
        }
    }
}

pub impl Repository {
    /// Get the path of this repository
    ///
    /// This is the path of the `.git` folder for normal repositories,
    /// or of the repository itself for bare repositories.
    fn path(&self) -> ~str {
        unsafe {
            let c_path = ext::git_repository_path(self.repo);
            str::raw::from_c_str(c_path)
        }
    }

    /// Get the path of the working directory for this repository
    ///
    /// If the repository is bare, this function will always return None.
    fn workdir(&self) -> Option<~str> {
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
    fn head(@mut self) -> ~Reference {
        unsafe {
            let mut ptr_to_ref: *ext::git_reference = ptr::null();

            if(ext::git_repository_head(&mut ptr_to_ref, self.repo) == 0) {
                ~Reference { c_ref: ptr_to_ref, repo_ptr: self }
            } else {
                raise!(conditions::bad_ref::cond)
            }
        }
    }

    /// Lookup a reference by name in a repository.
    /// The name will be checked for validity.
    fn lookup(@mut self, name: &str) -> Option<~Reference> {
        unsafe {
            let mut ptr_to_ref: *ext::git_reference = ptr::null();

            do str::as_c_str(name) |c_name| {
                if(ext::git_reference_lookup(&mut ptr_to_ref, self.repo, c_name) == 0) {
                    Some( ~Reference { c_ref: ptr_to_ref, repo_ptr: self } )
                } else {
                    None
                }
            }
        }
    }

    /// Lookup a commit object from repository
    fn lookup_commit(@mut self, id: &OID) -> Option<~Commit> {
        unsafe {
            let mut commit: *ext::git_commit = ptr::null();
            if ext::git_commit_lookup(&mut commit, self.repo, id) == 0 {
                Some( ~Commit { commit: commit, owner: self } )
            } else {
                None
            }
        }
    }

    /// Updates files in the index and the working tree to match the content of
    /// the commit pointed at by HEAD.
    /// This function does not accept options for now
    /// raise checkout_fail on error
    fn checkout_head(&mut self) {
        unsafe {
            if ext::git_checkout_head(self.repo, ptr::null()) != 0 {
                raise!(conditions::checkout_fail::cond)
            }
        }
    }

    /// Get the Index file for this repository.
    ///
    /// If a custom index has not been set, the default
    /// index for the repository will be returned (the one
    /// located in `.git/index`).
    fn index(@mut self) -> ~GitIndex {
        unsafe {
            let mut ptr_to_ref: *ext::git_index = ptr::null();

            if ext::git_repository_index(&mut ptr_to_ref, self.repo) == 0 {
                ~GitIndex { index: ptr_to_ref, owner: self }
            } else {
                raise!(conditions::bad_index::cond)
            }
        }
    }

    /// Check if a repository is empty
    fn is_empty(&self) -> bool {
        unsafe {
            let res = ext::git_repository_is_empty(self.repo);
            if res < 0 {
                raise!(conditions::check_fail::cond)
            } else {
                res as bool
            }
        }
    }

    /// Check if a repository is bare
    fn is_bare(&self) -> bool {
        unsafe {
            ext::git_repository_is_bare(self.repo) as bool
        }
    }

    /// Gather file statuses and run a callback for each one.
    /// The callback is passed the path of the file and the status (Status)
    /// If the callback returns false, this function will stop looping
    /// 
    /// return values:
    ///   Ok(true): the loop finished successfully
    ///   Ok(false): the callback returned false
    ///   Err(e): found libgit2 errors
    ///
    /// This method is unsafe, as it blocks other tasks while running
    unsafe fn each_status(&self,
                            op: &fn(path: ~str, status_flags: c_uint) -> bool)
                            -> bool
    {
        unsafe {
            let fptr: *c_void = cast::transmute(&op);
            let res = ext::git_status_foreach(self.repo, git_status_cb, fptr);
            if res == 0 {
                true
            } else if res == ext::GIT_EUSER {
                false
            } else {
                raise!(conditions::check_fail::cond)
            }
        }
    }

    /// Safer variant of each_status
    fn status(&self) -> ~[(~str, ~Status)] {
        let mut status_list:~[(~str, ~Status)] = ~[];
        for self.each_status |path, status_flags| {
            let status = ~Status {
                index_new: status_flags & ext::GIT_STATUS_INDEX_NEW != 0,
                index_modified: status_flags & ext::GIT_STATUS_INDEX_MODIFIED != 0,
                index_deleted: status_flags & ext::GIT_STATUS_INDEX_DELETED != 0,
                index_renamed: status_flags & ext::GIT_STATUS_INDEX_RENAMED != 0,
                index_typechange: status_flags & ext::GIT_STATUS_INDEX_TYPECHANGE != 0,
                wt_new: status_flags & ext::GIT_STATUS_WT_NEW != 0,
                wt_modified: status_flags & ext::GIT_STATUS_WT_MODIFIED != 0,
                wt_deleted: status_flags & ext::GIT_STATUS_WT_DELETED != 0,
                wt_typechange: status_flags & ext::GIT_STATUS_WT_TYPECHANGE != 0,
                ignored: status_flags & ext::GIT_STATUS_IGNORED != 0,
            };
            status_list.push((path, status));
        };
        status_list
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
    fn commit(&mut self, update_ref: Option<&str>, author: &Signature, committer: &Signature,
            message_encoding: Option<&str>, message: &str, tree: &Tree,
            parents: &[~Commit]) -> OID
    {
        unsafe {
            let c_ref = 
            match update_ref {
                None => ptr::null(),
                Some(uref) => str::as_c_str(uref, |ptr| {ptr}),
            };
            let c_author = signature::to_c_sig(author);
            let c_committer = signature::to_c_sig(committer);
            let c_encoding =
            match message_encoding {
                None => ptr::null(),
                Some(enc) => str::as_c_str(enc, |ptr| {ptr}),
            };
            let c_message = str::as_c_str(message, |ptr| {ptr});
            let mut oid = OID { id: [0, .. 20] };
            let c_parents = do vec::map(parents) |p| { p.commit };
            do vec::as_const_buf(c_parents) |parent_ptr, len| {
                let res = ext::git_commit_create(&mut oid, self.repo, c_ref,
                            &c_author, &c_committer, c_encoding, c_message, tree.tree,
                            len as c_int, parent_ptr);
                if res != 0 {
                    oid
                } else {
                    raise!(conditions::bad_oid::cond)
                }
            }
        }
    }
}

extern fn git_status_cb(path: *c_char, status_flags: c_uint, payload: *c_void) -> c_int
{
    unsafe {
        let op_ptr: *&fn(~str, c_uint) -> bool = cast::transmute(payload);
        let op = *op_ptr;
        let path_str = str::raw::from_c_str(path);
        if op(path_str, status_flags) {
            0
        } else {
            1
        }
    }
}

impl Drop for Repository {
    fn finalize(&self) {
        unsafe {
            ext::git_repository_free(self.repo);
        }
    }
}
