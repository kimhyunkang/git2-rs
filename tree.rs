use super::{Tree, OID};
use ext;

pub impl Tree {
    fn id(&self) -> &'self OID
    {
        unsafe {
            cast::transmute(ext::git_tree_id(self.tree))
        }
    }
}

#[unsafe_destructor]
impl Drop for Tree {
    fn finalize(&self) {
        unsafe {
            ext::git_tree_free(self.tree);
        }
    }
}
