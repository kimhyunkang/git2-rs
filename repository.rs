use core::libc::{c_char, c_void, c_int, c_uint, size_t};

use error::*;

static PATH_BUF_SZ: uint = 1024u;

// If we want to know actual storage size of git_repository,
// we have to know the storage size of pthread_mutex_t
// here we are going to treat git_repository as a large opaque object
type git_repository = c_void;

#[link_args = "-lgit2"]
extern "C" {
    /* from <git2/repository.h> */
    fn git_repository_open(out: **git_repository, path: *c_char) -> c_int;
    fn git_repository_free(repo: *git_repository) -> c_void;
    fn git_repository_discover(path_out: *mut c_char, path_size: size_t,
                            start_path: *c_char, across_fs: c_int,
                            ceiling_dirs: *c_char) -> c_int;
    fn git_repository_path(repo: *git_repository) -> *c_char;
    fn git_repository_init(out: **git_repository, path: *c_char, is_bare: c_uint) -> c_int;
}

pub struct Repository {
    priv repo: *git_repository,
}

pub fn open(path: &str) -> Result<@Repository, GitError>
{
    unsafe {
        let ptr_to_repo: *git_repository = ptr::null();
        let ptr2 = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(path) |c_path| {
            if(git_repository_open(ptr2, c_path) == 0) {
                Ok( @Repository { repo: ptr_to_repo } )
            } else {
                Err( err_last() )
            }
        }
    }
}

pub fn init(path: &str, is_bare: bool) -> Result<@Repository, GitError>
{
    unsafe {
        let ptr_to_repo: *git_repository = ptr::null();
        let ptr2 = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(path) |c_path| {
            if(git_repository_init(ptr2, c_path, is_bare as c_uint) == 0) {
                Ok( @Repository { repo: ptr_to_repo } )
            } else {
                Err( err_last() )
            }
        }
    }
}

pub fn discover(start_path: &str, across_fs: bool, ceiling_dirs: &str) 
    -> Result<~str, GitError>
{
    unsafe {
        let mut buf = vec::from_elem(PATH_BUF_SZ, 0u8 as c_char);
        do vec::as_mut_buf(buf) |c_path, sz| {
            do str::as_c_str(start_path) |c_start_path| {
                do str::as_c_str(ceiling_dirs) |c_ceiling_dirs| {
                    let result = git_repository_discover(c_path, sz as size_t,
                                            c_start_path, across_fs as c_int, c_ceiling_dirs);
                    if result == 0 {
                        let path_str = str::raw::from_buf(c_path as *u8);
                        Ok(path_str)
                    } else {
                        Err(err_last())
                    }
                }
            }
        }
    }
}

pub impl Repository {
    fn path(&self) -> ~str {
        unsafe {
            let c_path = git_repository_path(self.repo);
            str::raw::from_c_str(c_path)
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
