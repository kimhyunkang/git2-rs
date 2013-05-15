use ext;
use types::GitError;
use core::task::atomically;

pub fn atomic_err<T>(f: &fn() -> Option<T>) -> Result<T, GitError>
{
    unsafe {
        do atomically {
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
}
