use std::libc::c_int;
use std::str::raw::from_c_str;
use ext;
use super::{Signature, Time};

pub fn to_c_sig(sig: &Signature) -> ext::git_signature {
    do sig.name.as_c_str |c_name| {
        do sig.email.as_c_str |c_email| {
            ext::git_signature {
                name: c_name,
                email: c_email,
                when: ext::git_time {
                    time: sig.when.time,
                    offset: sig.when.offset as c_int,
                }
            }
        }
    }
}

pub unsafe fn from_c_sig(c_sig: *ext::git_signature) -> Signature {
    Signature {
        name: from_c_str((*c_sig).name),
        email: from_c_str((*c_sig).email),
        when: Time { time: (*c_sig).when.time, offset: (*c_sig).when.offset as int }
    }
}

#[inline]
fn time_cmp(a: &Time, b: &Time) -> i64 {
    let a_utc = a.time + (a.offset as i64) * 60;
    let b_utc = b.time + (b.offset as i64) * 60;
    return a_utc - b_utc;
}

impl Eq for Time {
    fn eq(&self, other: &Time) -> bool {
        time_cmp(self, other) == 0
    }

    fn ne(&self, other: &Time) -> bool {
        time_cmp(self, other) != 0
    }
}

impl Ord for Time {
    fn lt(&self, other: &Time) -> bool {
        time_cmp(self, other) < 0
    }

    fn le(&self, other: &Time) -> bool {
        time_cmp(self, other) <= 0
    }

    fn gt(&self, other: &Time) -> bool {
        time_cmp(self, other) > 0
    }

    fn ge(&self, other: &Time) -> bool {
        time_cmp(self, other) >= 0
    }
}

impl TotalOrd for Time {
    fn cmp(&self, other: &Time) -> Ordering {
        let res = time_cmp(self, other);
        if res < 0 {
            Less
        } else if res == 0 {
            Equal
        } else {
            Greater
        }
    }
}
