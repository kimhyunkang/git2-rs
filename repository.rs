use core::libc::{c_char, c_void, c_int};
use core::path::Path;

use error::*;

// If we want to know actual storage size of git_repository,
// we have to know the storage size of pthread_mutex_t
// here we are going to treat git_repository as a large opaque object
type git_repository = c_void;

#[link_args = "-lgit2"]
extern "C" {
    /* from <git2/repository.h> */
    fn git_repository_open(out: **git_repository, path: *c_char) -> c_int;
    fn git_repository_free(repo: *git_repository) -> c_void;
}

pub struct Repository {
    priv repo: *git_repository,
}

pub fn open(path: &Path) -> Result<@Repository, GitError>
{
    unsafe {
        let ptr_to_repo: *git_repository = ptr::null();
        let ptr2 = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(path.to_str()) |c_path| {
            if(git_repository_open(ptr2, c_path) == 0) {
                Ok( @Repository { repo: ptr_to_repo } )
            } else {
                Err( err_last() )
            }
        }
    }
}

impl Drop for Repository {
    fn finalize(&self) {
        unsafe {
            git_repository_free(self.repo);
        }
    }
}
