use super::{Blob, OID};
use ext;

pub impl Blob {
    /// get the id of the blob
    fn id(&self) -> &'self OID
    {
        unsafe {
            // OID pointer returned by git_blob_id is const pointer
            // so it's safe to use as long as self is alive
            cast::transmute(ext::git_blob_id(self.blob))
        }
    }

    ///
    /// Get a read-only buffer with the raw content of a blob.
    ///
    /// A reference to the raw content of a blob is transferred to closure
    fn rawcontent_as_slice<T>(&self, f: &fn(v: &[u8]) -> T) -> T
    {
        unsafe {
            let ptr:*u8 = cast::transmute(ext::git_blob_rawcontent(self.blob));
            let size = ext::git_blob_rawsize(self.blob);
            if(size < 0) {
                fail!(~"negative blob size")
            }
            vec::raw::buf_as_slice(ptr, size as uint, f)
        }
    }

    /// Determine if the blob content is most certainly binary or not.
    ///
    /// The heuristic used to guess if a file is binary is taken from core git:
    /// Searching for NUL bytes and looking for a reasonable ratio of printable
    /// to non-printable characters among the first 4000 bytes.
    fn is_binary(&self) -> bool
    {
        unsafe {
            ext::git_blob_is_binary(self.blob) as bool
        }
    }
}

#[unsafe_destructor]
impl Drop for Blob {
    fn finalize(&self) {
        unsafe {
            ext::git_blob_free(self.blob);
        }
    }
}
