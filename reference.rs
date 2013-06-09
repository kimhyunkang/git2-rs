use core::libc::c_char;
use super::{Reference, OID};
use ext;
use conditions;

macro_rules! raise {
    ($cond_expr:expr) => ({
        let err = ext::giterr_last();
        let message = str::raw::from_c_str((*err).message);
        let klass = (*err).klass;
        $cond_expr.raise((message, klass))
    })
}

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

    fn resolve(&self) -> OID {
        unsafe {
            let mut resolved_ref: *ext::git_reference = ptr::null();
            if ext::git_reference_resolve(&mut resolved_ref, self.c_ref) == 0 {
                let result_oid = ext::git_reference_target(resolved_ref);
                if result_oid == ptr::null() {
                    let err = ext::giterr_last();
                    let message = str::raw::from_c_str((*err).message);
                    let klass = (*err).klass;
                    ext::git_reference_free(resolved_ref);
                    conditions::bad_oid::cond.raise((message, klass))
                } else {
                    let mut oid = OID { id: [0, .. 20] };
                    ptr::copy_memory(&mut oid, result_oid, 1);
                    ext::git_reference_free(resolved_ref);
                    oid
                }
            } else {
                raise!(conditions::bad_oid::cond)
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
