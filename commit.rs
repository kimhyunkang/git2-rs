use std::libc::c_uint;
use std::ptr;
use std::str::raw::from_c_str;
use ext;
use signature;
use super::*;

impl Commit {
    /// get the id of the commit
    pub fn id<'r>(&self) -> &'r OID
    {
        unsafe {
            // OID pointer returned by git_commit_id is const pointer
            // so it's safe to use as long as self is alive
            ext::git_commit_id(self.commit)
        }
    }

    /// Get the encoding for the message of the commit,
    /// as a string representing a standard encoding name
    /// The encoding may be None, in that case UTF-8 is assumed
    pub fn message_encoding(&self) -> Option<~str>
    {
        unsafe {
            let encoding = ext::git_commit_message_encoding(self.commit);
            if encoding == ptr::null() {
                None
            } else {
                Some(from_c_str(encoding))
            }
        }
    }

    /// Get the full message of the commit
    pub fn message(&self) -> ~str
    {
        unsafe {
            let message = ext::git_commit_message(self.commit);
            from_c_str(message)
        }
    }

    /// Get the committer of a commit
    pub fn committer(&self) -> Signature
    {
        unsafe {
            let sig = ext::git_commit_committer(self.commit);
            signature::from_c_sig(sig)
        }
    }

    /// Get the author of a commit
    pub fn author(&self) -> Signature
    {
        unsafe {
            let sig = ext::git_commit_author(self.commit);
            signature::from_c_sig(sig)
        }
    }

    /// Get the tree pointed to by a commit.
    pub fn tree(&self) -> ~Tree
    {
        unsafe {
            let mut tree:*ext::git_tree = ptr::null();
            if ext::git_commit_tree(&mut tree, self.commit) == 0 {
                ~Tree { tree: tree, owner: self.owner }
            } else {
                fail!(~"failed to retrieve tree")
            }
        }
    }

    /// Get the parents of the commit.
    pub fn parents(&self) -> ~[~Commit]
    {
        unsafe {
            let len = ext::git_commit_parentcount(self.commit) as uint;
            let mut parents:~[~Commit] = std::vec::with_capacity(len);
            let mut success = true;
            do std::uint::iterate(0, len) |i| {
                let mut commit_ptr:*ext::git_commit = ptr::null();
                if ext::git_commit_parent(&mut commit_ptr, self.commit, i as c_uint) == 0 {
                    let commit = ~Commit { commit: commit_ptr, owner: self.owner };
                    parents.push(commit);
                } else {
                    raise();
                    success = false;
                };
                success
            };

            if success {
                return parents;
            } else {
                return ~[];
            }
        }
    }

    /// Get the commit object that is the <n>th generation ancestor
    /// of the commit object, following only the first parents.
    ///
    /// Passing `0` as the generation number returns another instance of the
    /// base commit itself.
    pub fn nth_gen_ancestor(&self, n: uint) -> Option<~Commit>
    {
        let mut ancestor: *ext::git_commit = ptr::null();
        unsafe {
            let res = ext::git_commit_parent(&mut ancestor, self.commit, n as c_uint);
            match res {
                0 => Some( ~Commit { commit: ancestor, owner: self.owner } ),
                ext::GIT_ENOTFOUND => None,
                _ => {
                    raise();
                    None
                },
            }
        }
    }

    /// Get the oid of parents for the commit. This is different from
    /// parents(&self), which will attempt to load the parent commit from the ODB.
    pub fn parents_oid(&self) -> ~[~OID]
    {
        unsafe {
            let len = ext::git_commit_parentcount(self.commit) as uint;
            let mut parents:~[~OID] = std::vec::with_capacity(len);
            let mut success = true;
            do std::uint::iterate(0, len) |i| {
                let mut oid = OID { id: [0, .. 20] };
                let res_ptr = ext::git_commit_parent_id(self.commit, i as c_uint);
                if res_ptr == ptr::null() {
                    raise();
                    success = false;
                } else {
                    ptr::copy_memory(&mut oid, res_ptr, 1);
                    parents.push(~oid);
                }
                success
            };

            if success {
                return parents;
            } else {
                return ~[];
            }
        }
    }
}

#[unsafe_destructor]
impl Drop for Commit {
    fn finalize(&self) {
        unsafe {
            ext::git_commit_free(self.commit);
        }
    }
}
