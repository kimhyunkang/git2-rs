use core::libc::{c_char, c_int, c_uint, size_t};
use ext;
use types::{GitError, Repository, Reference};

use error::*;

static PATH_BUF_SZ: uint = 1024u;

/// Open a git repository.
///
/// The 'path' argument must point to either a git repository
/// folder, or an existing work dir.
///
/// The method will automatically detect if 'path' is a normal
/// or bare repository or fail is 'path' is neither.
pub fn open(path: &str) -> Result<@Repository, GitError>
{
    unsafe {
        let ptr_to_repo: *ext::git_repository = ptr::null();
        let ptr2 = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(path) |c_path| {
            do atomic_err {
                if(ext::git_repository_open(ptr2, c_path) == 0) {
                    Some( @Repository { repo: ptr_to_repo } )
                } else {
                    None
                }
            }
        }
    }
}

/// Creates a new Git repository in the given folder.
/// if is_bare is true, a Git repository without a working directory is
/// created at the pointed path. If false, provided path will be
/// considered as the working directory into which the .git directory
/// will be created.
pub fn init(path: &str, is_bare: bool) -> Result<@Repository, GitError>
{
    unsafe {
        let ptr_to_repo: *ext::git_repository = ptr::null();
        let ptr2 = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(path) |c_path| {
            do atomic_err {
                if(ext::git_repository_init(ptr2, c_path, is_bare as c_uint) == 0) {
                    Some( @Repository { repo: ptr_to_repo } )
                } else {
                    None
                }
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
/// might be NULL (which is equivalent to an empty string)
pub fn discover(start_path: &str, across_fs: bool, ceiling_dirs: &str) 
    -> Result<~str, GitError>
{
    unsafe {
        let mut buf = vec::from_elem(PATH_BUF_SZ, 0u8 as c_char);
        do vec::as_mut_buf(buf) |c_path, sz| {
            do str::as_c_str(start_path) |c_start_path| {
                do str::as_c_str(ceiling_dirs) |c_ceiling_dirs| {
                    do atomic_err {
                        let result = ext::git_repository_discover(c_path, sz as size_t,
                                                c_start_path, across_fs as c_int, c_ceiling_dirs);
                        if result == 0 {
                            let path_str = str::raw::from_buf(c_path as *u8);
                            Some(path_str)
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}

/// Clone a remote repository, and checkout the branch pointed to by the remote
/// this function do not receive options for now
pub fn clone(url: &str, local_path: &str) -> Result<@Repository, GitError> {
    unsafe {
        let ptr_to_repo: *ext::git_repository = ptr::null();
        let pptr = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(url) |c_url| {
            do str::as_c_str(local_path) |c_path| {
                do atomic_err {
                    if ext::git_clone(pptr, c_url, c_path, ptr::null()) == 0 {
                        Some( @Repository { repo: ptr_to_repo } )
                    } else {
                        None
                    }
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
    fn head(@self) -> Result<~Reference, GitError> {
        unsafe {
            let ptr_to_ref: *ext::git_reference = ptr::null();
            let pptr = ptr::to_unsafe_ptr(&ptr_to_ref);

            do atomic_err {
                if(ext::git_repository_head(pptr, self.repo) == 0) {
                    Some( ~Reference { c_ref: ptr_to_ref, repo_ptr: self } )
                } else {
                    None
                }
            }
        }
    }

    /// Check if a repository is empty
    fn is_empty(&self) -> bool {
        unsafe {
            let res = ext::git_repository_is_empty(self.repo);
            if res < 0 {
                fail!(~"repository is corrupted")
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
}

impl Drop for Repository {
    fn finalize(&self) {
        unsafe {
            ext::git_repository_free(self.repo);
        }
    }
}
