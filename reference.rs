use std::libc::{c_char, c_int};
use std::{ptr, str, c_str};
use super::{OID, raise};
use super::Repository;
use ext;

/// Delete the branch reference.
pub fn branch_delete(reference: &Reference) {
    unsafe {
        if ext::git_branch_delete(reference.c_ref) != 0 {
            raise();
        }
    }
}

pub struct Reference<'r> {
    priv c_ref: *ext::git_reference,
    priv owner: &'r Repository,
}

impl<'r> Reference<'r> {
    pub fn new(c_ref: *ext::git_reference, owner: &'r Repository) -> Reference<'r> {
        Reference {
            c_ref: c_ref,
            owner: owner,
        }
    }

    ///
    /// Return the name of the given local or remote branch.
    ///
    /// The name of the branch matches the definition of the name for branch_lookup.
    /// That is, if the returned name is given to branch_lookup() then the reference is
    /// returned that was given to this function.
    ///
    /// return Some(~str) on success; otherwise None (if the ref is no local or remote branch).
    ///
    pub fn branch_name(&self) -> Option<~str> {
        unsafe {
            let mut ptr_to_name: *c_char = ptr::null();
            if ext::git_branch_name(&mut ptr_to_name, self.c_ref) == 0 {
                Some(str::raw::from_c_str(ptr_to_name))
            } else {
                None
            }
        }
    }

    /// Determine if the current local branch is pointed at by HEAD.
    pub fn is_head(&self) -> bool {
        unsafe {
            match ext::git_branch_is_head(self.c_ref) {
                1 => true,
                0 => false,
                _ => { raise(); false },
            }
        }
    }

    /// Move/rename an existing local branch reference.
    ///
    /// The new branch name will be checked for validity.
    /// See `git_tag_create()` for rules about valid names.
    pub fn branch_move(&self, new_branch_name: &str, force: bool) -> Option<Reference<'r>>
    {
        let mut ptr: *ext::git_reference = ptr::null();
        let flag = force as c_int;
        unsafe {
            new_branch_name.with_c_str(|c_name| {
                let res = ext::git_branch_move(&mut ptr, self.c_ref, c_name, flag);
                match res {
                    0 => Some( Reference::new(ptr, self.owner) ),
                    ext::GIT_EINVALIDSPEC => None,
                    _ => { raise(); None },
                }
            })
        }
    }

    /// Return the reference supporting the remote tracking branch,
    /// returns None when the upstream is not found
    pub fn upstream(&self) -> Option<Reference<'r>>
    {
        let mut ptr: *ext::git_reference = ptr::null();
        unsafe {
            let res = ext::git_branch_upstream(&mut ptr, self.c_ref);
            match res {
                0 => Some( Reference::new(ptr, self.owner) ),
                ext::GIT_ENOTFOUND => None,
                _ => { raise(); None },
            }
        }
    }

    /// Set the upstream configuration for a given local branch
    /// upstream_name: remote-tracking or local branch to set as
    ///     upstream. Pass None to unset.
    pub fn set_upstream(&self, upstream_name: Option<&str>)
    {
        unsafe {
            let c_name =
            match upstream_name {
                None => c_str::CString::new(ptr::null(), false),
                Some(nameref) => nameref.to_c_str(),
            };

            c_name.with_ref(|name_ptr| {
                if ext::git_branch_set_upstream(self.c_ref, name_ptr) == 0 {
                    ()
                } else {
                    raise()
                }
            })
        }
    }

    pub fn resolve(&self) -> OID {
        unsafe {
            let mut resolved_ref: *ext::git_reference = ptr::null();
            let mut oid = OID { id: [0, .. 20] };
            if ext::git_reference_resolve(&mut resolved_ref, self.c_ref) == 0 {
                let result_oid = ext::git_reference_target(resolved_ref);
                if result_oid == ptr::null() {
                    raise();
                } else {
                    ptr::copy_memory(&mut oid, result_oid, 1);
                    ext::git_reference_free(resolved_ref);
                }
            } else {
                raise();
            }
            return oid;
        }
    }
}

#[unsafe_destructor]
impl<'r> Drop for Reference<'r> {
    fn drop(&mut self) {
        unsafe {
            ext::git_reference_free(self.c_ref);
        }
    }
}
