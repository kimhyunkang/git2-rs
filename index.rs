use types::{GitIndex, GitError};
use ext;

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
    /// @param index an existing index object
    /// @param path filename to add
    /// Returns None on success, Some(GitError) on error
    pub fn add_bypath(&self, path: &str) -> Option<GitError> {
        unsafe {
            do str::as_c_str(path) |c_path| {
                do task::atomically {
                    if ext::git_index_add_bypath(self.index, c_path) == 0 {
                        None
                    } else {
                        let err = ext::giterr_last();
                        Some(GitError {
                                message: str::raw::from_c_str((*err).message),
                                klass: (*err).klass,
                            })
                    }
                }
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
