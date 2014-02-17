use super::OID;
use std::cast;
use std::vec::raw::buf_as_slice;
use super::Repository;
use ext;

pub struct Blob<'r> {
    priv blob: *ext::git_blob,
    priv owner: &'r Repository,
}

impl<'r> Blob<'r> {
    // TODO: make this function priv
    pub fn new(blob: *ext::git_blob, owner: &'r Repository) -> Blob<'r>
    {
        Blob {
            blob: blob,
            owner: owner
        }
    }

    /// get the id of the blob
    pub fn id<'r>(&self) -> &'r OID
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
    pub fn rawcontent_as_slice<T>(&self, f: |v: &[u8]| -> T) -> T
    {
        unsafe {
            let ptr:*u8 = cast::transmute(ext::git_blob_rawcontent(self.blob));
            let size = ext::git_blob_rawsize(self.blob);
            if(size < 0) {
                fail!(~"negative blob size")
            }
            buf_as_slice(ptr, size as uint, f)
        }
    }

    /// Determine if the blob content is most certainly binary or not.
    ///
    /// The heuristic used to guess if a file is binary is taken from core git:
    /// Searching for NUL bytes and looking for a reasonable ratio of printable
    /// to non-printable characters among the first 4000 bytes.
    pub fn is_binary(&self) -> bool
    {
        unsafe {
            ext::git_blob_is_binary(self.blob) != 0
        }
    }
}

#[unsafe_destructor]
impl<'r> Drop for Blob<'r> {
    fn drop(&mut self) {
        unsafe {
            ext::git_blob_free(self.blob);
        }
    }
}
