use core::libc::{c_char, c_int, c_uint, size_t};
use ext;
use types::{GitError, Repository, Reference};

use error::*;

static PATH_BUF_SZ: uint = 1024u;

pub fn open(path: &str) -> Result<@Repository, GitError>
{
    unsafe {
        let ptr_to_repo: *ext::git_repository = ptr::null();
        let ptr2 = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(path) |c_path| {
            if(ext::git_repository_open(ptr2, c_path) == 0) {
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
        let ptr_to_repo: *ext::git_repository = ptr::null();
        let ptr2 = ptr::to_unsafe_ptr(&ptr_to_repo);
        do str::as_c_str(path) |c_path| {
            if(ext::git_repository_init(ptr2, c_path, is_bare as c_uint) == 0) {
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
                    let result = ext::git_repository_discover(c_path, sz as size_t,
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
            let c_path = ext::git_repository_path(self.repo);
            str::raw::from_c_str(c_path)
        }
    }

    fn head(@self) -> Result<~Reference, GitError> {
        unsafe {
            let ptr_to_ref: *ext::git_reference = ptr::null();
            let pptr = ptr::to_unsafe_ptr(&ptr_to_ref);

            if(ext::git_repository_head(pptr, self.repo) == 0) {
                Ok( ~Reference { c_ref: ptr_to_ref, repo_ptr: self } )
            } else {
                Err( err_last() )
            }
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
