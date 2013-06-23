use core::libc::{c_char, c_int, c_uint, c_void, size_t};
use ext;
use signature;
use super::*;

static PATH_BUF_SZ: uint = 1024u;

/// Open a git repository.
///
/// The 'path' argument must point to either a git repository folder, or an existing work dir.
///
/// The method will automatically detect if 'path' is a normal
/// or bare repository or raise bad_repo if 'path' is neither.
pub fn open(path: &str) -> Result<@mut Repository, (~str, GitError)>
{
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(path) |c_path| {
            if ext::git_repository_open(&mut ptr_to_repo, c_path) == 0 {
                Ok( @mut Repository { repo: ptr_to_repo } )
            } else {
                Err( last_error() )
            }
        }
    }
}

/// Open a bare repository on the serverside.
///
/// This is a fast open for bare repositories that will come in handy
/// if you're e.g. hosting git repositories and need to access them
/// efficiently
pub fn open_bare(path: &str) -> Result<@mut Repository, (~str, GitError)>
{
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(path) |c_path| {
            if ext::git_repository_open_bare(&mut ptr_to_repo, c_path) == 0 {
                Ok( @mut Repository { repo: ptr_to_repo } )
            } else {
                Err( last_error() )
            }
        }
    }
}

/// Creates a new Git repository in the given folder.
/// if is_bare is true, a Git repository without a working directory is
/// created at the pointed path. If false, provided path will be
/// considered as the working directory into which the .git directory
/// will be created.
pub fn init(path: &str, is_bare: bool) -> Result<@mut Repository, (~str, GitError)>
{
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(path) |c_path| {
            if ext::git_repository_init(&mut ptr_to_repo, c_path, is_bare as c_uint) == 0 {
                Ok( @mut Repository { repo: ptr_to_repo } )
            } else {
                Err( last_error() )
            }
        }
    }
}

/// Look for a git repository and copy its path in the given buffer.
/// The lookup start from base_path and walk across parent directories
/// if nothing has been found. The lookup ends when the first repository
/// is found, or when reaching a directory referenced in ceiling_dirs
/// or when the filesystem changes (in case across_fs is true).
///
/// The method will automatically detect if the repository is bare
/// (if there is a repository).
///
/// ceiling_dirs: A GIT_PATH_LIST_SEPARATOR separated list of
/// absolute symbolic link free paths. The lookup will stop when any
/// of this paths is reached. Note that the lookup always performs on
/// start_path no matter start_path appears in ceiling_dirs ceiling_dirs
/// might be empty string
pub fn discover(start_path: &str, across_fs: bool, ceiling_dirs: &str) -> Option<~str>
{
    unsafe {
        let mut buf = vec::from_elem(PATH_BUF_SZ, 0u8 as c_char);
        do vec::as_mut_buf(buf) |c_path, sz| {
            do str::as_c_str(start_path) |c_start_path| {
                do str::as_c_str(ceiling_dirs) |c_ceiling_dirs| {
                    let result = ext::git_repository_discover(c_path, sz as size_t,
                                            c_start_path, across_fs as c_int, c_ceiling_dirs);
                    if result == 0 {
                        Some( str::raw::from_buf(c_path as *u8) )
                    } else {
                        None
                    }
                }
            }
        }
    }
}

/// Clone a remote repository, and checkout the branch pointed to by the remote
/// this function do not receive options for now
pub fn clone(url: &str, local_path: &str) -> Result<@mut Repository, (~str, GitError)> {
    unsafe {
        let mut ptr_to_repo: *ext::git_repository = ptr::null();
        do str::as_c_str(url) |c_url| {
            do str::as_c_str(local_path) |c_path| {
                if ext::git_clone(&mut ptr_to_repo, c_url, c_path, ptr::null()) == 0 {
                    Ok( @mut Repository { repo: ptr_to_repo } )
                } else {
                    Err( last_error() )
                }
            }
        }
    }
}

impl Repository {
    /// Get the path of this repository
    ///
    /// This is the path of the `.git` folder for normal repositories,
    /// or of the repository itself for bare repositories.
    pub fn path(&self) -> ~str {
        unsafe {
            let c_path = ext::git_repository_path(self.repo);
            str::raw::from_c_str(c_path)
        }
    }

    /// Get the path of the working directory for this repository
    ///
    /// If the repository is bare, this function will always return None.
    pub fn workdir(&self) -> Option<~str> {
        unsafe {
            let c_path = ext::git_repository_workdir(self.repo);
            if ptr::is_null(c_path) {
                None
            } else {
                Some(str::raw::from_c_str(c_path))
            }
        }
    }

    /// Retrieve and resolve the reference pointed at by HEAD.
    pub fn head(@mut self) -> Option<~Reference> {
        unsafe {
            let mut ptr_to_ref: *ext::git_reference = ptr::null();

            match ext::git_repository_head(&mut ptr_to_ref, self.repo) {
                0 => Some( ~Reference { c_ref: ptr_to_ref, repo_ptr: self } ),
                ext::GIT_EORPHANEDHEAD => None,
                ext::GIT_ENOTFOUND => None,
                _ => {
                    raise();
                    None
                },
            }
        }
    }

    /// Lookup a reference by name in a repository.
    /// The name will be checked for validity.
    pub fn lookup(@mut self, name: &str) -> Option<~Reference> {
        unsafe {
            let mut ptr_to_ref: *ext::git_reference = ptr::null();

            do str::as_c_str(name) |c_name| {
                if(ext::git_reference_lookup(&mut ptr_to_ref, self.repo, c_name) == 0) {
                    Some( ~Reference { c_ref: ptr_to_ref, repo_ptr: self } )
                } else {
                    None
                }
            }
        }
    }

    /// Lookup a branch by its name in a repository.
    ///
    /// The generated reference must be freed by the user.
    ///
    /// The branch name will be checked for validity.
    /// See `git_tag_create()` for rules about valid names.
    ///
    /// Returns None if the branch name is invalid, or the branch is not found
    ///
    /// remote: True if you want to consider remote branch,
    ///     or false if you want to consider local branch
    pub fn lookup_branch(@mut self, branch_name: &str, remote: bool) -> Option<~Reference> {
        let mut ptr: *ext::git_reference = ptr::null();
        let branch_type = if remote { ext::GIT_BRANCH_REMOTE } else { ext::GIT_BRANCH_LOCAL };
        do str::as_c_str(branch_name) |c_name| {
            unsafe {
                let res = ext::git_branch_lookup(&mut ptr, self.repo, c_name, branch_type);
                match res {
                    0 => Some( ~Reference { c_ref: ptr, repo_ptr: self } ),
                    ext::GIT_ENOTFOUND => None,
                    ext::GIT_EINVALIDSPEC => None,
                    _ => { raise(); None },
                }
            }
        }
    }

    /// Lookup a commit object from repository
    pub fn lookup_commit(@mut self, id: &OID) -> Option<~Commit> {
        unsafe {
            let mut commit: *ext::git_commit = ptr::null();
            if ext::git_commit_lookup(&mut commit, self.repo, id) == 0 {
                Some( ~Commit { commit: commit, owner: self } )
            } else {
                None
            }
        }
    }

    /// Lookup a tree object from repository
    pub fn lookup_tree(@mut self, id: &OID) -> Option<~Tree> {
        unsafe {
            let mut tree: *ext::git_tree = ptr::null();
            if ext::git_tree_lookup(&mut tree, self.repo, id) == 0 {
                Some( ~Tree { tree: tree, owner: self } )
            } else {
                None
            }
        }
    }

    /// Updates files in the index and the working tree to match the content of
    /// the commit pointed at by HEAD.
    /// This function does not accept options for now
    ///
    /// returns true when successful, false if HEAD points to an non-existing branch
    /// raise on other errors
    pub fn checkout_head(&mut self) -> bool {
        unsafe {
            match ext::git_checkout_head(self.repo, ptr::null()) {
                0 => true,
                ext::GIT_EORPHANEDHEAD => false,
                _ => {
                    raise();
                    false
                }
            }
        }
    }

    /// Get the Index file for this repository.
    ///
    /// If a custom index has not been set, the default
    /// index for the repository will be returned (the one
    /// located in `.git/index`).
    pub fn index(@mut self) -> Result<~GitIndex, (~str, GitError)> {
        unsafe {
            let mut ptr_to_ref: *ext::git_index = ptr::null();

            if ext::git_repository_index(&mut ptr_to_ref, self.repo) == 0 {
                Ok( ~GitIndex { index: ptr_to_ref, owner: self } )
            } else {
                Err( last_error() )
            }
        }
    }

    /// Check if a repository is empty
    pub fn is_empty(&self) -> bool {
        unsafe {
            let res = ext::git_repository_is_empty(self.repo);
            if res < 0 {
                raise();
                false
            } else {
                res as bool
            }
        }
    }

    /// Check if a repository is bare
    pub fn is_bare(&self) -> bool {
        unsafe {
            ext::git_repository_is_bare(self.repo) as bool
        }
    }

    /// Gather file statuses and run a callback for each one.
    /// The callback is passed the path of the file and the status (Status)
    /// If the callback returns false, this function will stop looping
    /// 
    /// return values:
    ///   Ok(true): the loop finished successfully
    ///   Ok(false): the callback returned false
    ///   Err(e): found libgit2 errors
    ///
    /// This method is unsafe, as it blocks other tasks while running
    pub unsafe fn each_status(&self,
                            op: &fn(path: ~str, status_flags: c_uint) -> bool)
                            -> bool
    {
        unsafe {
            let fptr: *c_void = cast::transmute(&op);
            let res = ext::git_status_foreach(self.repo, git_status_cb, fptr);
            if res == 0 {
                true
            } else if res == ext::GIT_EUSER {
                false
            } else {
                raise();
                false
            }
        }
    }

    /// Safer variant of each_status
    pub fn status(&self) -> ~[(~str, ~Status)] {
        let mut status_list:~[(~str, ~Status)] = ~[];
        for self.each_status |path, status_flags| {
            let status = ~Status {
                index_new: status_flags & ext::GIT_STATUS_INDEX_NEW != 0,
                index_modified: status_flags & ext::GIT_STATUS_INDEX_MODIFIED != 0,
                index_deleted: status_flags & ext::GIT_STATUS_INDEX_DELETED != 0,
                index_renamed: status_flags & ext::GIT_STATUS_INDEX_RENAMED != 0,
                index_typechange: status_flags & ext::GIT_STATUS_INDEX_TYPECHANGE != 0,
                wt_new: status_flags & ext::GIT_STATUS_WT_NEW != 0,
                wt_modified: status_flags & ext::GIT_STATUS_WT_MODIFIED != 0,
                wt_deleted: status_flags & ext::GIT_STATUS_WT_DELETED != 0,
                wt_typechange: status_flags & ext::GIT_STATUS_WT_TYPECHANGE != 0,
                ignored: status_flags & ext::GIT_STATUS_IGNORED != 0,
            };
            status_list.push((path, status));
        };
        status_list
    }


    /// Create a new branch pointing at a target commit
    ///
    /// A new direct reference will be created pointing to
    /// this target commit. If `force` is true and a reference
    /// already exists with the given name, it'll be replaced.
    ///
    /// The returned reference must be freed by the user.
    ///
    /// The branch name will be checked for validity.
    /// See `git_tag_create()` for rules about valid names.
    pub fn branch_create(@mut self, branch_name: &str, target: &Commit, force: bool)
        -> Option<~Reference>
    {
        let mut ptr: *ext::git_reference = ptr::null();
        let flag = force as c_int;
        unsafe {
            do str::as_c_str(branch_name) |c_name| {
                let res = ext::git_branch_create(&mut ptr, self.repo, c_name, target.commit, flag);
                match res {
                    0 => Some( ~Reference { c_ref: ptr, repo_ptr: self } ),
                    ext::GIT_EINVALIDSPEC => None,
                    _ => { raise(); None },
                }
            }
        }
    }

    /// Loop over all the branches and issue a callback for each one.
    pub fn branch_foreach(&self, local: bool, remote: bool,
        op: &fn(name: &str, is_remote: bool) -> bool) -> bool
    {
        let flocal = if local { ext::GIT_BRANCH_LOCAL } else { 0 };
        let fremote = if remote { ext::GIT_BRANCH_REMOTE } else { 0 };
        let flags = flocal & fremote;
        unsafe {
            let payload: *c_void = cast::transmute(&op);
            let res = ext::git_branch_foreach(self.repo, flags, git_branch_foreach_cb, payload);
            match res {
                0 => true,
                ext::GIT_EUSER => false,
                _ => { raise(); false },
            }
        }
    }

    /// Return the name of the reference supporting the remote tracking branch,
    /// given the name of a local branch reference.
    pub fn upstream_name(&self, canonical_branch_name: &str) -> Option<~str>
    {
        let mut buf: [c_char, ..1024] = [0, ..1024];
        do str::as_c_str(canonical_branch_name) |c_name| {
            do vec::as_mut_buf(buf) |v, _len| {
                unsafe {
                    let res = ext::git_branch_upstream_name(v, 1024, self.repo, c_name);
                    if res >= 0 {
                        let ptr: *c_char = cast::transmute(v);
                        Some( str::raw::from_c_str_len(ptr, res as uint) )
                    } else if res == ext::GIT_ENOTFOUND {
                        None
                    } else {
                        raise();
                        None
                    }
                }
            }
        }
    }

    /// Return the name of remote that the remote tracking branch belongs to.
    /// returns Err(GIT_ENOTFOUND) when no remote matching remote was found,
    /// returns Err(GIT_EAMBIGUOUS) when the branch maps to several remotes,
    pub fn git_branch_remote_name(&self, canonical_branch_name: &str)
        -> Result<~str, (~str, GitError)>
    {
        let mut buf: [c_char, ..1024] = [0, ..1024];
        do str::as_c_str(canonical_branch_name) |c_name| {
            do vec::as_mut_buf(buf) |v, _len| {
                unsafe {
                    let res = ext::git_branch_remote_name(v, 1024, self.repo, c_name);
                    if res >= 0 {
                        let ptr: *c_char = cast::transmute(v);
                        Ok( str::raw::from_c_str_len(ptr, res as uint) )
                    } else {
                        Err( last_error() )
                    }
                }
            }
        }
    }

    /// Lookup a blob object from a repository.
    pub fn blob_lookup(@mut self, id: &OID) -> Option<~Blob>
    {
        let mut ptr: *ext::git_blob = ptr::null();
        unsafe {
            if ext::git_blob_lookup(&mut ptr, self.repo, id) == 0 {
                Some( ~Blob { blob: ptr, owner: self } )
            } else {
                None
            }
        }
    }

    /// Read a file from the working folder of a repository
    /// and write it to the Object Database as a loose blob
    pub fn blob_create_fromworkdir(@mut self, relative_path: &str) -> Result<~Blob, (~str, GitError)>
    {
        let mut oid = OID { id: [0, ..20] };
        let mut ptr: *ext::git_blob = ptr::null();
        do str::as_c_str(relative_path) |c_path| {
            unsafe {
                if ext::git_blob_create_fromworkdir(&mut oid, self.repo, c_path) == 0 {
                    if ext::git_blob_lookup(&mut ptr, self.repo, &oid) != 0 {
                        fail!(~"blob lookup failure");
                    }
                    Ok( ~Blob { blob: ptr, owner: self } )
                } else {
                    Err( last_error() )
                }
            }
        }
    }

    /// Read a file from the filesystem and write its content
    /// to the Object Database as a loose blob
    pub fn blob_create_fromdisk(@mut self, relative_path: &str) -> Result<~Blob, (~str, GitError)>
    {
        let mut oid = OID { id: [0, ..20] };
        let mut ptr: *ext::git_blob = ptr::null();
        do str::as_c_str(relative_path) |c_path| {
            unsafe {
                if ext::git_blob_create_fromdisk(&mut oid, self.repo, c_path) == 0 {
                    if ext::git_blob_lookup(&mut ptr, self.repo, &oid) != 0 {
                        fail!(~"blob lookup failure");
                    }
                    Ok( ~Blob { blob: ptr, owner: self } )
                } else {
                    Err( last_error() )
                }
            }
        }
    }

    /// Write a loose blob to the Object Database from a
    /// provider of chunks of data.
    ///
    /// Provided the `hintpath` parameter is not None, its value
    /// will help to determine what git filters should be applied
    /// to the object before it can be placed to the object database.
    pub fn blob_create_fromreader(@mut self, reader: &io::Reader, hintpath: Option<&str>)
        -> Result<~Blob, (~str, GitError)>
    {
        let mut oid = OID { id: [0, ..20] };
        unsafe {
            let c_path =
            match hintpath {
                None => ptr::null(),
                Some(pathref) => str::as_c_str(pathref, |ptr| {ptr}),
            };
            let payload: *c_void = cast::transmute(&reader);
            if (ext::git_blob_create_fromchunks(&mut oid, self.repo, c_path, git_blob_chunk_cb,
                    payload) == 0) {
                let mut ptr: *ext::git_blob = ptr::null();
                if ext::git_blob_lookup(&mut ptr, self.repo, &oid) != 0 {
                    fail!(~"blob lookup failure");
                }
                Ok( ~Blob { blob: ptr, owner: self } )
            } else {
                Err( last_error() )
            }
        }
    }

    /// Write an in-memory buffer to the ODB as a blob
    pub fn blob_create_frombuffer(@mut self, buffer: &[u8]) -> Result<~Blob, (~str, GitError)>
    {
        let mut oid = OID { id: [0, ..20] };
        do vec::as_imm_buf(buffer) |v, len| {
            unsafe {
                let buf:*c_void = cast::transmute(v);
                if ext::git_blob_create_frombuffer(&mut oid, self.repo, buf, len as u64) == 0 {
                    let mut ptr: *ext::git_blob = ptr::null();
                    if ext::git_blob_lookup(&mut ptr, self.repo, &oid) != 0 {
                        fail!(~"blob lookup failure");
                    }
                    Ok( ~Blob { blob: ptr, owner: self } )
                } else {
                    Err( last_error() )
                }
            }
        }
    }

    /// Create new commit in the repository from a list of Commit pointers
    ///
    /// Returns the created commit. The commit will be written to the Object Database and
    ///  the given reference will be updated to point to it
    ///
    /// id: Pointer in which to store the OID of the newly created commit
    ///
    /// update_ref: If not None, name of the reference that
    ///  will be updated to point to this commit. If the reference
    ///  is not direct, it will be resolved to a direct reference.
    ///  Use "HEAD" to update the HEAD of the current branch and
    ///  make it point to this commit. If the reference doesn't
    ///  exist yet, it will be created.
    ///
    /// author: Signature with author and author time of commit
    ///
    /// committer: Signature with committer and commit time of commit
    ///
    /// message_encoding: The encoding for the message in the
    ///  commit, represented with a standard encoding name.
    ///  E.g. "UTF-8". If None, no encoding header is written and
    ///  UTF-8 is assumed.
    ///
    /// message: Full message for this commit
    ///
    /// tree: An instance of a Tree object that will
    ///  be used as the tree for the commit. This tree object must
    ///  also be owned by `self`
    ///
    /// parents: Vector of Commit objects that will be used as the parents for this commit.
    ///  All the given commits must be owned by `self`.
    pub fn commit(&mut self, update_ref: Option<&str>, author: &Signature, committer: &Signature,
            message_encoding: Option<&str>, message: &str, tree: &Tree,
            parents: &[~Commit]) -> OID
    {
        unsafe {
            let c_ref = 
            match update_ref {
                None => ptr::null(),
                Some(uref) => str::as_c_str(uref, |ptr| {ptr}),
            };
            let c_author = signature::to_c_sig(author);
            let c_committer = signature::to_c_sig(committer);
            let c_encoding =
            match message_encoding {
                None => ptr::null(),
                Some(enc) => str::as_c_str(enc, |ptr| {ptr}),
            };
            let c_message = str::as_c_str(message, |ptr| {ptr});
            let mut oid = OID { id: [0, .. 20] };
            let c_parents = do vec::map(parents) |p| { p.commit };
            do vec::as_const_buf(c_parents) |parent_ptr, len| {
                let res = ext::git_commit_create(&mut oid, self.repo, c_ref,
                            &c_author, &c_committer, c_encoding, c_message, tree.tree,
                            len as c_int, parent_ptr);
                if res != 0 {
                    raise()
                }
                oid
            }
        }
    }
}

extern fn git_status_cb(path: *c_char, status_flags: c_uint, payload: *c_void) -> c_int
{
    unsafe {
        let op_ptr: *&fn(~str, c_uint) -> bool = cast::transmute(payload);
        let op = *op_ptr;
        let path_str = str::raw::from_c_str(path);
        if op(path_str, status_flags) {
            0
        } else {
            1
        }
    }
}

extern fn git_blob_chunk_cb(content: *mut u8, max_length: size_t, payload: *&io::Reader) -> c_int
{
    let len = max_length as uint;
    unsafe {
        let reader = *payload;
        do vec::raw::mut_buf_as_slice(content, len) |v| {
            if reader.eof() {
                0
            } else {
                reader.read(v, len) as c_int
            }
        }
    }
}

extern fn git_branch_foreach_cb(branch_name: *c_char, branch_type: ext::git_branch_t,
    payload: *c_void) -> c_int
{
    unsafe {
        let op_ptr: *&fn(name: &str, is_remote: bool) -> bool = cast::transmute(payload);
        let op = *op_ptr;
        let branch_str = str::raw::from_c_str(branch_name);
        let is_remote = (branch_type == ext::GIT_BRANCH_REMOTE);
        if op(branch_str, is_remote) {
            0
        } else {
            1
        }
    }
}

impl Drop for Repository {
    fn finalize(&self) {
        unsafe {
            ext::git_repository_free(self.repo);
        }
    }
}
