extern mod std;

use ext;
use super::Signature;

pub fn to_c_sig(sig: &Signature) -> ext::git_signature {
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

pub unsafe fn from_c_sig(c_sig: *ext::git_signature) -> Signature {
    let spec = std::time::Timespec::new((*c_sig).when.time, 0);
    let mut tm = std::time::at_utc(spec);
    tm.tm_gmtoff = (*c_sig).when.offset * 60;
    Signature {
        name: str::raw::from_c_str((*c_sig).name),
        email: str::raw::from_c_str((*c_sig).email),
        when: tm,
    }
}
