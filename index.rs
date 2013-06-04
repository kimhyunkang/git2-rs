use types::GitIndex;
use ext;

#[unsafe_destructor]
impl Drop for GitIndex {
    fn finalize(&self) {
        unsafe {
            ext::git_index_free(self.index);
        }
    }
}
