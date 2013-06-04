use types::Tree;
use ext;

#[unsafe_destructor]
impl Drop for Tree {
    fn finalize(&self) {
        unsafe {
            ext::git_tree_free(self.tree);
        }
    }
}
