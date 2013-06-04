use ext;

pub struct Repository {
    pub repo: *ext::git_repository,
}

pub struct Reference {
    pub c_ref: *ext::git_reference,
    pub repo_ptr: @Repository,
}

pub struct GitIndex {
    pub index: *ext::git_index,
    pub owner: @Repository,
}

pub struct Tree {
    pub tree: *ext::git_tree,
    pub owner: @Repository,
}

/// Status flags for a single file.
///
/// A combination of these values will be returned to indicate the status of a file.
/// Status compares the working directory, the index, and the current HEAD of the repository.
/// The `index` set of flags represents the status of file in the index relative to the HEAD,
/// and the `wt` set of flags represent the status of the file in the working directory
/// relative to the index.
pub struct Status {
    pub index_new: bool,
    pub index_modified: bool,
    pub index_deleted: bool,
    pub index_renamed: bool,
    pub index_typechange: bool,

    pub wt_new: bool,
    pub wt_modified: bool,
    pub wt_deleted: bool,
    pub wt_typechange: bool,

    pub ignored: bool,
}
