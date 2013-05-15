use ext;
use types::GitError;

pub fn err_last() -> GitError {
    unsafe {
        let err = ext::giterr_last();
        GitError {
            message: str::raw::from_c_str((*err).message),
            klass: (*err).klass,
        }
    }
}

