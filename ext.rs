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
pub static GIT_OK:c_int = 0;
pub static GIT_ERROR:c_int = -1;
pub static GIT_ENOTFOUND:c_int = -3;
pub static GIT_EEXISTS:c_int = -4;
pub static GIT_EAMBIGUOUS:c_int = -5;
pub static GIT_EBUFS:c_int = -6;
pub static GIT_EUSER:c_int = -7;
pub static GIT_EBAREREPO:c_int = -8;
pub static GIT_EORPHANEDHEAD:c_int = -9;
pub static GIT_EUNMERGED:c_int = -10;
pub static GIT_ENONFASTFORWARD:c_int = -11;
pub static GIT_EINVALIDSPEC:c_int = -12;
pub static GIT_EMERGECONFLICT:c_int = -13;

pub static GIT_PASSTHROUGH:c_int = -30;
pub static GIT_ITEROVER:c_int = -31;

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
type git_checkout_strategy_t = uint;

/** default is a dry run, no actual updates */
static GIT_CHECKOUT_NONE:git_checkout_strategy_t = 0;

/** Allow safe updates that cannot overwrite uncommitted data */
static GIT_CHECKOUT_SAFE:git_checkout_strategy_t = (1u << 0);

/** Allow safe updates plus creation of missing files */
static GIT_CHECKOUT_SAFE_CREATE:git_checkout_strategy_t = (1u << 1);

/** Allow all updates to force working directory to look like index */
static GIT_CHECKOUT_FORCE:git_checkout_strategy_t = (1u << 2);

/** Allow checkout to make safe updates even if conflicts are found */
static GIT_CHECKOUT_ALLOW_CONFLICTS:git_checkout_strategy_t = (1u << 4);

/** Remove untracked files not in index (that are not ignored) */
static GIT_CHECKOUT_REMOVE_UNTRACKED:git_checkout_strategy_t = (1u << 5);

/** Remove ignored files not in index */
static GIT_CHECKOUT_REMOVE_IGNORED:git_checkout_strategy_t = (1u << 6);

/** Only update existing files, don't create new ones */
static GIT_CHECKOUT_UPDATE_ONLY:git_checkout_strategy_t = (1u << 7);

/** Normally checkout updates index entries as it goes; this stops that */
static GIT_CHECKOUT_DONT_UPDATE_INDEX:git_checkout_strategy_t = (1u << 8);

/** Don't refresh index/config/etc before doing checkout */
static GIT_CHECKOUT_NO_REFRESH:git_checkout_strategy_t = (1u << 9);

/** Treat pathspec as simple list of exact match file paths */
static GIT_CHECKOUT_DISABLE_PATHSPEC_MATCH:git_checkout_strategy_t = (1u << 13);

/** Ignore directories in use, they will be left empty */
static GIT_CHECKOUT_SKIP_LOCKED_DIRECTORIES:git_checkout_strategy_t = (1u << 18);

/**
 * THE FOLLOWING OPTIONS ARE NOT YET IMPLEMENTED
 */

/** Allow checkout to skip unmerged files (NOT IMPLEMENTED) */
static GIT_CHECKOUT_SKIP_UNMERGED:git_checkout_strategy_t = (1u << 10);
/** For unmerged files, checkout stage 2 from index (NOT IMPLEMENTED) */
static GIT_CHECKOUT_USE_OURS:git_checkout_strategy_t = (1u << 11);
/** For unmerged files, checkout stage 3 from index (NOT IMPLEMENTED) */
static GIT_CHECKOUT_USE_THEIRS:git_checkout_strategy_t = (1u << 12);

/** Recursively checkout submodules with same options (NOT IMPLEMENTED) */
static GIT_CHECKOUT_UPDATE_SUBMODULES:git_checkout_strategy_t = (1u << 16);
/** Recursively checkout submodules if HEAD moved in super repo (NOT IMPLEMENTED) */
static GIT_CHECKOUT_UPDATE_SUBMODULES_IF_CHANGED:git_checkout_strategy_t = (1u << 17);

type git_checkout_notify_t = uint;

static GIT_CHECKOUT_NOTIFY_NONE:git_checkout_notify_t       = 0;
static GIT_CHECKOUT_NOTIFY_CONFLICT:git_checkout_notify_t   = (1u << 0);
static GIT_CHECKOUT_NOTIFY_DIRTY:git_checkout_notify_t      = (1u << 1);
static GIT_CHECKOUT_NOTIFY_UPDATED:git_checkout_notify_t    = (1u << 2);
static GIT_CHECKOUT_NOTIFY_UNTRACKED:git_checkout_notify_t  = (1u << 3);
static GIT_CHECKOUT_NOTIFY_IGNORED:git_checkout_notify_t    = (1u << 4);

/* from <git2/checkout.h> */
pub struct git_checkout_opts {
    version: c_uint,

    checkout_strategy: git_checkout_strategy_t,

    disable_filters: c_int,
    dir_mode: c_uint,
    file_mode: c_uint,
    file_open_flags: c_int,

    notify_flags: git_checkout_notify_t,
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

/* from <git2/status.h> */
pub static GIT_STATUS_INDEX_NEW:c_uint        = (1u << 0) as c_uint;
pub static GIT_STATUS_INDEX_MODIFIED:c_uint   = (1u << 1) as c_uint;
pub static GIT_STATUS_INDEX_DELETED:c_uint    = (1u << 2) as c_uint;
pub static GIT_STATUS_INDEX_RENAMED:c_uint    = (1u << 3) as c_uint;
pub static GIT_STATUS_INDEX_TYPECHANGE:c_uint = (1u << 4) as c_uint;

pub static GIT_STATUS_WT_NEW:c_uint           = (1u << 7) as c_uint;
pub static GIT_STATUS_WT_MODIFIED:c_uint      = (1u << 8) as c_uint;
pub static GIT_STATUS_WT_DELETED:c_uint       = (1u << 9) as c_uint;
pub static GIT_STATUS_WT_TYPECHANGE:c_uint    = (1u << 10) as c_uint;

pub static GIT_STATUS_IGNORED:c_uint          = (1u << 14) as c_uint;

/* from <git2/tree.h> */
pub enum git_treewalk_mode {
	GIT_TREEWALK_PRE = 0, /* Pre-order */
	GIT_TREEWALK_POST = 1, /* Post-order */
}

/* from <git2/types.h> */

// the storage size of these types are unknown
pub type git_repository = c_void;
pub type git_reference = c_void;
pub type git_tree = c_void;
pub type git_tree_entry = c_void;
pub type git_treebuilder = c_void;
pub type git_index = c_void;
pub type git_commit = c_void;
pub type git_object = c_void;

#[cfg(target_os = "android")]
#[cfg(target_os = "freebsd")]
#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
pub type git_time_t = i64;

#[cfg(target_os = "win32")]
pub type git_time_t = core::libc::types::os::arch::extra::time64_t;

pub struct git_time {
    time: git_time_t,
    offset: c_int,
}

pub struct git_signature {
    pub name: *c_char,
    pub email: *c_char,
    pub when: git_time,
}

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
    pub fn git_repository_index(out: **git_index, repo: *git_repository) -> c_int;

    /* from <git2/refs.h> */
    pub fn git_reference_free(c_ref: *git_reference) -> c_void;
    pub fn git_reference_lookup(out: **git_reference, repo: *git_repository,
                                name: *c_char) -> c_int;
    pub fn git_reference_resolve(out: &mut *git_reference, c_ref: *git_reference) -> c_int;
    pub fn git_reference_target(c_ref: *git_reference) -> *super::OID;

    /* from <git2/threads.h> */
    pub fn git_threads_init() -> c_void;
    pub fn git_threads_shutdown() -> c_void;

    /* from <git2/clone.h> */
    pub fn git_clone(out: **git_repository, url: *c_char, local_path: *c_char,
                    options: *git_clone_options) -> c_int;

    /* from <git2/checkout.h> */
    pub fn git_checkout_head(repo: *git_repository, opts: *git_checkout_opts) -> c_int;

    /* from <git2/index.h> */
    pub fn git_index_free(index: *git_index) -> c_void;
    pub fn git_index_write(index: *git_index) -> c_int;
    pub fn git_index_write_tree(out: *super::OID, index: *git_index) -> c_int;
    pub fn git_index_add_bypath(index: *git_index, path: *c_char) -> c_int;
    pub fn git_index_remove_bypath(index: *git_index, path: *c_char) -> c_int;
    pub fn git_index_read_tree(index: *git_index, tree: *git_tree) -> c_int;
    pub fn git_index_clear(index: *git_index) -> c_void;

    /* from <git2/status.h> */
    pub fn git_status_foreach(repo: *git_repository, callback: callback_t,
                                payload: *c_void) -> c_int;

    /* from <git2/branch.h> */
    pub fn git_branch_name(out: **c_char, c_ref: *git_reference) -> c_int;

    /* from <git2/object.h> */
    pub fn git_object_free(object: *git_object) -> c_void;
    pub fn git_object_id(obj: *git_object) -> *super::OID;
    pub fn git_object_lookup(out: &mut *git_object, repo: *git_repository, id: *super::OID,
        otype: super::OType) -> c_int;

    /* from <git2/oid.h> */
    pub fn git_oid_fromstr(out: &mut super::OID, c_str: *c_char) -> c_int;
    pub fn git_oid_fmt(out: *mut c_char, oid: &super::OID) -> c_int;

    /* from <git2/commit.h> */
    pub fn git_commit_message_encoding(commit: *git_commit) -> *c_char;
    pub fn git_commit_message(commit: *git_commit) -> *c_char;
    pub fn git_commit_committer(commit: *git_commit) -> *git_signature;
    pub fn git_commit_author(commit: *git_commit) -> *git_signature;
    pub fn git_commit_tree(tree_out: &mut *git_tree, commit: *git_commit) -> c_int;
    pub fn git_commit_parentcount(commit: *git_commit) -> c_uint;
    pub fn git_commit_parent(out: &mut *git_commit, commit: *git_commit, n: c_uint) -> c_int;
    pub fn git_commit_parent_id(commit: *git_commit, n: c_uint) -> *super::OID;
    pub fn git_commit_create(id: &mut super::OID, repo: *git_repository,
        update_ref: *c_char, author: &git_signature, committer: &git_signature,
        message_encoding: *c_char, message: *c_char, tree: *git_tree,
        parent_count: c_int, parents: *const *git_commit) -> c_int;

    /* from <git2/tree.h> */
    pub fn git_tree_id(tree: *git_tree) -> *super::OID;
    pub fn git_tree_entrycount(tree: *git_tree) -> size_t;
    pub fn git_tree_entry_byname(tree: *git_tree, filename: *c_char) -> *git_tree_entry;
    pub fn git_tree_entry_byindex(tree: *git_tree, idx: size_t) -> *git_tree_entry;
    pub fn git_tree_entry_byoid(tree: *git_tree, oid: &super::OID) -> *git_tree_entry;
    pub fn git_tree_entry_bypath(out: &mut *git_tree_entry, tree: *git_tree,
        path: *c_char) -> c_int;
    pub fn git_tree_entry_dup(entry: *git_tree_entry) -> *git_tree_entry;
    pub fn git_tree_entry_free(entry: *git_tree_entry) -> c_void;
    pub fn git_tree_entry_name(entry: *git_tree_entry) -> *c_char;
    pub fn git_tree_entry_id(entry: *git_tree_entry) -> *super::OID;
    pub fn git_tree_entry_type(entry: *git_tree_entry) -> super::OType;
    pub fn git_tree_entry_filemode(entry: *git_tree_entry) -> super::FileMode;
    pub fn git_tree_entry_cmp(e1: *git_tree_entry, e2: *git_tree_entry) -> c_int;
    pub fn git_treebuilder_create(out: &mut *git_treebuilder, source: *git_tree) -> c_int;
    pub fn git_treebuilder_clear(bld: *git_treebuilder) -> c_void;
    pub fn git_treebuilder_entrycount(bld: *git_treebuilder) -> c_uint;
    pub fn git_treebuilder_free(bld: *git_treebuilder) -> c_void;
    pub fn git_treebuilder_get(bld: *git_treebuilder, filename: *c_char) -> *git_tree_entry;
    pub fn git_treebuilder_insert(out: &mut *git_tree_entry, bld: *git_treebuilder,
        filename: *c_char, id: &super::OID, filemode: super::FileMode) -> c_int;
    pub fn git_treebuilder_remove(bld: *git_treebuilder, filename: *c_char) -> c_int;
    pub fn git_treebuilder_filter(bld: *git_treebuilder, filter: callback_t,
        payload: *c_void) -> c_void;
    pub fn git_treebuilder_write(id: &mut super::OID, repo: *git_repository,
        bld: *git_treebuilder) -> c_int;
    pub fn git_tree_walk(tree: *git_tree, mode: git_treewalk_mode, callback: callback_t,
        payload: *c_void) -> c_int;
}

/* from <git2/commit.h> */
#[inline]
pub unsafe fn git_commit_lookup(commit: &mut *git_commit, repo: *git_repository,
        id: &super::OID) -> c_int
{
	git_object_lookup(commit, repo, id, super::GIT_OBJ_COMMIT)
}

#[inline]
pub unsafe fn git_commit_free(commit: *git_commit) -> c_void
{
    git_object_free(commit)
}

#[inline]
pub unsafe fn git_commit_id(commit: *git_commit) -> *super::OID
{
    git_object_id(commit)
}

/* from <git2/tree.h> */
#[inline]
pub unsafe fn git_tree_free(tree: *git_tree) -> c_void
{
    git_object_free(tree)
}

#[inline]
pub unsafe fn git_tree_lookup(out: &mut *git_tree, repo: *git_repository, id: *super::OID) -> c_int
{
    git_object_lookup(out, repo, id, super::GIT_OBJ_TREE)
}
