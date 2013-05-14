use core::libc::{c_char};

/* from <git2/errors.h> */
enum git_return_code {
    GIT_OK = 0,
    GIT_ERROR = -1,
    GIT_ENOTFOUND = -3,
    GIT_EEXISTS = -4,
    GIT_EAMBIGUOUS = -5,
    GIT_EBUFS = -6,
    GIT_EUSER = -7,
    GIT_EBAREREPO = -8,
    GIT_EORPHANEDHEAD = -9,
    GIT_EUNMERGED = -10,
    GIT_ENONFASTFORWARD = -11,
    GIT_EINVALIDSPEC = -12,
    GIT_EMERGECONFLICT = -13,

    GIT_PASSTHROUGH = -30,
    GIT_ITEROVER = -31,
}

/** Error classes */
pub enum git_error_t {
    GITERR_NOMEMORY,
    GITERR_OS,
    GITERR_INVALID,
    GITERR_REFERENCE,
    GITERR_ZLIB,
    GITERR_REPOSITORY,
    GITERR_CONFIG,
    GITERR_REGEX,
    GITERR_ODB,
    GITERR_INDEX,
    GITERR_OBJECT,
    GITERR_NET,
    GITERR_TAG,
    GITERR_TREE,
    GITERR_INDEXER,
    GITERR_SSL,
    GITERR_SUBMODULE,
    GITERR_THREAD,
    GITERR_STASH,
    GITERR_CHECKOUT,
    GITERR_FETCHHEAD,
    GITERR_MERGE,
}

struct git_error {
    pub message: *c_char,
    pub klass: git_error_t,
}

#[link_args = "-lgit2"]
extern "C" {
    /* from <git2/errors.h> */
    fn giterr_last() -> *git_error;
}

pub struct GitError {
    pub message: ~str,
    pub klass: git_error_t,
}

pub fn err_last() -> GitError {
    unsafe {
        let err = giterr_last();
        GitError {
            message: str::raw::from_c_str((*err).message),
            klass: (*err).klass,
        }
    }
}

