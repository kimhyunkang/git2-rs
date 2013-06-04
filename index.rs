use types::{GitIndex, Tree};
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

impl GitIndex {
    /// Add or update an index entry from a file on disk
    ///
    /// The file `path` must be relative to the repository's
    /// working folder and must be readable.
    ///
    /// This method will fail in bare index instances.
    ///
    /// This forces the file to be added to the index, not looking
    /// at gitignore rules.  Those rules can be evaluated through
    /// the status APIs before calling this.
    ///
    /// If this file currently is the result of a merge conflict, this
    /// file will no longer be marked as conflicting.  The data about
    /// the conflict will be moved to the "resolve undo" (REUC) section.
    ///
    /// raises index_fail on error
    pub fn add_bypath(&mut self, path: &str) {
        unsafe {
            do str::as_c_str(path) |c_path| {
                if ext::git_index_add_bypath(self.index, c_path) != 0 {
                    raise!(conditions::index_fail::cond);
                }
            }
        }
    }

    /// Remove an index entry corresponding to a file on disk
    ///
    /// The file `path` must be relative to the repository's working folder.  It may exist.
    ///
    /// If this file currently is the result of a merge conflict, this
    /// file will no longer be marked as conflicting.  The data about
    /// the conflict will be moved to the "resolve undo" (REUC) section.
    ///
    /// raises index_fail on error
    pub fn remove_bypath(&mut self, path: &str) {
        unsafe {
            do str::as_c_str(path) |c_path| {
                if ext::git_index_remove_bypath(self.index, c_path) != 0 {
                    raise!(conditions::index_fail::cond);
                }
            }
        }
    }

    /// Write an existing index object from memory back to disk using an atomic file lock.
    ///
    /// raises index_fail on error
    pub fn write(&self) -> ~Tree {
        unsafe {
            let oid = ext::git_oid { id: [0, .. 20] };
            let oid_ptr = ptr::to_unsafe_ptr(&oid);
            if ext::git_index_write_tree(oid_ptr, self.index) == 0 {
                let ptr_to_tree: *ext::git_tree = ptr::null();
                let pptr = ptr::to_unsafe_ptr(&ptr_to_tree);
                if ext::git_tree_lookup(pptr, self.owner.repo, oid_ptr) == 0 {
                    ~Tree { tree: ptr_to_tree, owner: self.owner }
                } else {
                    raise!(conditions::bad_tree::cond)
                }
            } else {
                raise!(conditions::bad_tree::cond)
            }
        }
    }
}

#[unsafe_destructor]
impl Drop for GitIndex {
    fn finalize(&self) {
        unsafe {
            ext::git_index_free(self.index);
        }
    }
}
