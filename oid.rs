use core::libc::c_char;

use super::OID;
use ext;
use conditions;
use core::{from_str, to_str};

fn from_str(s: &str) -> OID {
    unsafe {
        let mut oid = OID { id: [0, .. 20] };
        do str::as_c_str(s) |c_str| {
            if ext::git_oid_fromstr(&mut oid, c_str) == 0 {
                oid
            } else {
                let err = ext::giterr_last();
                let message = str::raw::from_c_str((*err).message);
                let klass = (*err).klass;
                conditions::bad_oid::cond.raise((message, klass))
            }
        }
    }
}

impl from_str::FromStr for OID {
    fn from_str(s: &str) -> Option<OID> {
        unsafe {
            let mut oid = OID { id: [0, .. 20] };
            do str::as_c_str(s) |c_str| {
                if ext::git_oid_fromstr(&mut oid, c_str) == 0 {
                    Some(oid)
                } else {
                    None
                }
            }
        }
    }
}

impl to_str::ToStr for OID {
    fn to_str(&self) -> ~str {
        let mut v: ~[c_char] = vec::with_capacity(41);
        unsafe {
            do vec::as_mut_buf(v) |vbuf, _len| {
                ext::git_oid_fmt(vbuf, self)
            };
            vec::raw::set_len(&mut v, 40);
            v.push(0);

            return cast::transmute(v);
        }
    }
}
