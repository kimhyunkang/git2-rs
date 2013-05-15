use core::libc::{c_char, c_void, c_int, c_uint, size_t};

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

/* from <git2/errors.h> */
pub struct git_error {
    pub message: *c_char,
    pub klass: git_error_t,
}

/* from <git2/errors.h> */
pub enum git_return_code {
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

/* from <git2/types.h> */
// the storage size of these types are unknown
pub type git_repository = c_void;
pub type git_reference = c_void;

#[link_args = "-lgit2"]
pub extern {
    /* from <git2/errors.h> */
    pub fn giterr_last() -> *git_error;

    /* from <git2/repository.h> */
    pub fn git_repository_open(out: **git_repository, path: *c_char) -> c_int;
    pub fn git_repository_free(repo: *git_repository) -> c_void;
    pub fn git_repository_discover(path_out: *mut c_char, path_size: size_t,
                            start_path: *c_char, across_fs: c_int,
                            ceiling_dirs: *c_char) -> c_int;
    pub fn git_repository_path(repo: *git_repository) -> *c_char;
    pub fn git_repository_init(out: **git_repository, path: *c_char, is_bare: c_uint) -> c_int;
    pub fn git_repository_head(out: **git_reference, repo: *git_repository) -> c_int;

    /* from <git2/refs.h> */
    pub fn git_reference_free(c_ref: *git_reference) -> c_void;
}
