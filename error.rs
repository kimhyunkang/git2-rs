use ext;

pub struct GitError {
    pub message: ~str,
    pub klass: ext::git_error_t,
}

pub fn err_last() -> GitError {
    unsafe {
        let err = ext::giterr_last();
        GitError {
            message: str::raw::from_c_str((*err).message),
            klass: (*err).klass,
        }
    }
}

