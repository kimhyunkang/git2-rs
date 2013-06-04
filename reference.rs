use types::Reference;
use ext;


#[unsafe_destructor]
impl Drop for Reference {
    fn finalize(&self) {
        unsafe {
            ext::git_reference_free(self.c_ref);
        }
    }
}
