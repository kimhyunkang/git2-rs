use core::libc::c_char;
use types::Reference;
use ext;

pub impl Reference {
    ///
    /// Return the name of the given local or remote branch.
    ///
    /// The name of the branch matches the definition of the name for branch_lookup.
    /// That is, if the returned name is given to branch_lookup() then the reference is
    /// returned that was given to this function.
    ///
    /// return Some(~str) on success; otherwise None (if the ref is no local or remote branch).
    ///
    fn branch_name(&self) -> Option<~str> {
        unsafe {
            let ptr_to_name: *c_char = ptr::null();
            let pptr = ptr::to_unsafe_ptr(&ptr_to_name);
            if ext::git_branch_name(pptr, self.c_ref) == 0 {
                Some(str::raw::from_c_str(ptr_to_name))
            } else {
                None
            }
        }
    }
}

#[unsafe_destructor]
impl Drop for Reference {
    fn finalize(&self) {
        unsafe {
            ext::git_reference_free(self.c_ref);
        }
    }
}
