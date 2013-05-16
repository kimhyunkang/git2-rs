use ext;
use types::GitError;

/// if a git function can fail, call with this function to get GitError object 
pub unsafe fn atomic_err<T>(f: &fn() -> Option<T>) -> Result<T, GitError>
{
    do task::atomically {
        match f() {
            None => {
                let err = ext::giterr_last();
                Err(GitError {
                        message: str::raw::from_c_str((*err).message),
                        klass: (*err).klass,
                    })
            },
            Some(T) => Ok(T)
        }
    }
}
