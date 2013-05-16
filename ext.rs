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

/* from <git2/remote.h> */
pub enum git_remote_autotag_option_t {
    GIT_REMOTE_DOWNLOAD_TAGS_UNSET,
    GIT_REMOTE_DOWNLOAD_TAGS_NONE,
    GIT_REMOTE_DOWNLOAD_TAGS_AUTO,
    GIT_REMOTE_DOWNLOAD_TAGS_ALL
}

/* from <git2/transport.h> */
pub struct git_transport {
    set_callbacks: callback_t,
    connect: callback_t,
    ls: callback_t,
    push: callback_t,
    negotiate_fetch: callback_t,
    download_pack: callback_t,
    is_connected: callback_t,
    read_flags: callback_t,
    cancel: callback_t,
    close: callback_t,
    free: callback_t,
}

/* from <git2/strarray.h> */
pub struct git_strarray {
    strings: **c_char,
    count: size_t,
}

/* from <git2/checkout.h> */
pub struct git_checkout_opts {
    version: c_uint,

    checkout_strategy: c_uint,

    disable_filters: c_int,
    dir_mode: c_uint,
    file_mode: c_uint,
    file_open_flags: c_int,

    notify_flags: c_uint,
    notify_cb: callback_t,
    notify_payload: *c_void,

    progress_cb: callback_t,
    progress_payload: *c_void,

    paths: git_strarray,

    baseline: *git_tree,
}

/* from <git2/clone.h> */
pub struct git_clone_options {
    version: c_uint,

    checkout_opts: git_checkout_opts,
    bare: c_int,
    fetch_progress_cb: callback_t,
    fetch_progress_payload: *c_void,

    remote_name: *c_char,
    pushurl: *c_char,
    fetch_spec: *c_char,
    push_spec: *c_char,
    cred_acquire_cb: callback_t,
    cred_acquire_payload: *c_void,
    transport: *git_transport,
    remote_callbacks: callback_t,
    remote_autotag: git_remote_autotag_option_t,
    checkout_branch: *c_char,
}

/* from <git2/types.h> */
// the storage size of these types are unknown
pub type git_repository = c_void;
pub type git_reference = c_void;
pub type git_tree = c_void;

// value type of 'crust' functions is *u8
pub type callback_t = *u8;

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
    pub fn git_repository_workdir(repo: *git_repository) -> *c_char;
    pub fn git_repository_init(out: **git_repository, path: *c_char, is_bare: c_uint) -> c_int;
    pub fn git_repository_head(out: **git_reference, repo: *git_repository) -> c_int;
    pub fn git_repository_is_empty(repo: *git_repository) -> c_int;
    pub fn git_repository_is_bare(repo: *git_repository) -> c_int;

    /* from <git2/refs.h> */
    pub fn git_reference_free(c_ref: *git_reference) -> c_void;
    pub fn git_reference_lookup(out: **git_reference, repo: *git_repository,
                                name: *c_char) -> c_int;

    /* from <git2/threads.h> */
    pub fn git_threads_init() -> c_void;
    pub fn git_threads_shutdown() -> c_void;

    /* from <git2/clone.h> */
    pub fn git_clone(out: **git_repository, url: *c_char, local_path: *c_char,
                    options: *git_clone_options) -> c_int;
}
