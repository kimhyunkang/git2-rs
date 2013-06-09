extern mod std;

use core::libc::c_uint;
use conditions;
use ext;
use super::{Commit, Signature, OID, Tree};

macro_rules! raise {
    ($cond_expr:expr) => ({
        let err = ext::giterr_last();
        let message = str::raw::from_c_str((*err).message);
        let klass = (*err).klass;
        $cond_expr.raise((message, klass))
    })
}

fn to_c_sig(sig: &Signature) -> ext::git_signature {
    let ts = sig.when.to_timespec();
    do str::as_c_str(sig.name) |c_name| {
        do str::as_c_str(sig.email) |c_email| {
            ext::git_signature {
                name: c_name,
                email: c_email,
                when: ext::git_time {
                    time: ts.sec,
                    offset: sig.when.tm_gmtoff / 60,
                }
            }
        }
    }
}

unsafe fn from_c_sig(c_sig: *ext::git_signature) -> Signature {
    let spec = std::time::Timespec::new((*c_sig).when.time, 0);
    let mut tm = std::time::at_utc(spec);
    tm.tm_gmtoff = (*c_sig).when.offset * 60;
    Signature {
        name: str::raw::from_c_str((*c_sig).name),
        email: str::raw::from_c_str((*c_sig).email),
        when: tm,
    }
}

pub impl Commit {
    /// get the id of the commit
    fn id(&self) -> OID
    {
        let mut oid = OID { id: [0, ..20] };
        unsafe {
            let result_ptr = ext::git_commit_id(self.commit);
            ptr::copy_memory(&mut oid, result_ptr, 1);
        }
        return oid;
    }

    /// Get the encoding for the message of the commit,
    /// as a string representing a standard encoding name
    /// The encoding may be None, in that case UTF-8 is assumed
    fn message_encoding(&self) -> Option<~str>
    {
        unsafe {
            let encoding = ext::git_commit_message_encoding(self.commit);
            if encoding == ptr::null() {
                None
            } else {
                Some(str::raw::from_c_str(encoding))
            }
        }
    }

    /// Get the full message of the commit
    fn message(&self) -> ~str
    {
        unsafe {
            let message = ext::git_commit_message(self.commit);
            str::raw::from_c_str(message)
        }
    }

    /// Get the committer of a commit
    fn committer(&self) -> Signature
    {
        unsafe {
            let sig = ext::git_commit_committer(self.commit);
            from_c_sig(sig)
        }
    }

    /// Get the author of a commit
    fn author(&self) -> Signature
    {
        unsafe {
            let sig = ext::git_commit_author(self.commit);
            from_c_sig(sig)
        }
    }

    fn tree(&self) -> ~Tree
    {
        unsafe {
            let mut tree:*ext::git_tree = ptr::null();
            if ext::git_commit_tree(&mut tree, self.commit) == 0 {
                ~Tree { tree: tree, owner: self.owner }
            } else {
                raise!(conditions::bad_tree::cond)
            }
        }
    }

    fn parents(&self) -> ~[~Commit]
    {
        unsafe {
            let len = ext::git_commit_parentcount(self.commit) as uint;
            let mut parents:~[~Commit] = vec::with_capacity(len);
            for uint::range(0, len) |i| {
                let mut commit_ptr:*ext::git_commit = ptr::null();
                let commit =
                if ext::git_commit_parent(&mut commit_ptr, self.commit, i as c_uint) == 0 {
                    ~Commit { commit: commit_ptr, owner: self.owner }
                } else {
                    raise!(conditions::bad_commit::cond)
                };

                parents.push(commit)
            }

            return parents;
        }
    }

    fn parents_oid(&self) -> ~[~OID]
    {
        unsafe {
            let len = ext::git_commit_parentcount(self.commit) as uint;
            let mut parents:~[~OID] = vec::with_capacity(len);
            for uint::range(0, len) |i| {
                let mut oid = OID { id: [0, .. 20] };
                let res_ptr = ext::git_commit_parent_id(self.commit, i as c_uint);
                if res_ptr == ptr::null() {
                    let err = ext::giterr_last();
                    let message = str::raw::from_c_str((*err).message);
                    let klass = (*err).klass;
                    let trap_oid = conditions::bad_oid::cond.raise((message, klass));
                    parents.push(~trap_oid);
                } else {
                    ptr::copy_memory(&mut oid, res_ptr, 1);
                    parents.push(~oid);
                }
            }

            return parents;
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
