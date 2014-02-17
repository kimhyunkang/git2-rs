use std::{to_str, str};
use super::OID;
use ext;

impl FromStr for OID {
    fn from_str(s: &str) -> Option<OID> {
        unsafe {
            let mut oid = OID { id: [0, .. 20] };
            s.with_c_str(|c_str| {
                if ext::git_oid_fromstr(&mut oid, c_str) == 0 {
                    Some(oid)
                } else {
                    None
                }
            })
        }
    }
}

impl to_str::ToStr for OID {
    fn to_str(&self) -> ~str {
        let mut buf = [0 as u8, ..40];
        unsafe {
            ext::git_oid_fmt(buf.as_mut_ptr(), self);
        }
        str::from_utf8(buf).to_owned()
    }
}

/* from <git2/oid.h> */
#[inline]
fn git_oid_cmp(a: &OID, b: &OID) -> int {
    let mut idx = 0u;
    while idx < 20u {
        if a.id[idx] != b.id[idx] {
            return (a.id[idx] as int) - (b.id[idx] as int)
        }
        idx += 1;
    }
    return 0;
}

impl Eq for OID {
    fn eq(&self, other: &OID) -> bool {
        git_oid_cmp(self, other) == 0
    }

    fn ne(&self, other: &OID) -> bool {
        git_oid_cmp(self, other) != 0
    }
}

impl Ord for OID {
    fn lt(&self, other: &OID) -> bool {
        git_oid_cmp(self, other) < 0
    }

    fn le(&self, other: &OID) -> bool {
        git_oid_cmp(self, other) <= 0
    }

    fn gt(&self, other: &OID) -> bool {
        git_oid_cmp(self, other) > 0
    }

    fn ge(&self, other: &OID) -> bool {
        git_oid_cmp(self, other) >= 0
    }
}

impl TotalEq for OID {
    fn equals(&self, other: &OID) -> bool {
        git_oid_cmp(self, other) == 0
    }
}

impl TotalOrd for OID {
    fn cmp(&self, other: &OID) -> Ordering {
        let cmp = git_oid_cmp(self, other);
        if cmp < 0 {
            Less
        } else if cmp == 0 {
            Equal
        } else {
            Greater
        }
    }
}
