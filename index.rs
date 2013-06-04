use types::GitIndex;
use ext;

use conditions;

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
                    let err = ext::giterr_last();
                    let message = str::raw::from_c_str((*err).message);
                    let klass = (*err).klass;
                    conditions::index_fail::cond.raise((message, klass))
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
                    let err = ext::giterr_last();
                    let message = str::raw::from_c_str((*err).message);
                    let klass = (*err).klass;
                    conditions::index_fail::cond.raise((message, klass))
                }
            }
        }
    }

    /// Write an existing index object from memory back to disk using an atomic file lock.
    ///
    /// raises index_fail on error
    pub fn write(&self) -> ext::git_oid {
        unsafe {
            let oid = ext::git_oid { id: [0, .. 20] };
            let oid_ptr = ptr::to_unsafe_ptr(&oid);
            if ext::git_index_write_tree(oid_ptr, self.index) == 0 {
                oid 
            } else {
                let err = ext::giterr_last();
                let message = str::raw::from_c_str((*err).message);
                let klass = (*err).klass;
                conditions::oid_fail::cond.raise((message, klass))
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
