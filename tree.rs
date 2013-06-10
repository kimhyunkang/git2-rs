use core::libc::{size_t, c_void, c_char, c_int};
use super::*;
use conditions;
use ext;

macro_rules! raise {
    ($cond_expr:expr) => ({
        let err = ext::giterr_last();
        let message = str::raw::from_c_str((*err).message);
        let klass = (*err).klass;
        $cond_expr.raise((message, klass))
    })
}

pub impl Tree {
    /// Get the id of a tree.
    fn id(&self) -> &'self OID
    {
        unsafe {
            cast::transmute(ext::git_tree_id(self.tree))
        }
    }

    /// Lookup a tree entry by its filename
    fn entry_byname(&self, filename: &str) -> Option<~TreeEntry>
    {
        do str::as_c_str(filename) |c_filename| {
            unsafe {
                let entry_ptr = ext::git_tree_entry_byname(self.tree, c_filename);
                if entry_ptr == ptr::null() {
                    None
                } else {
                    Some( ~TreeEntry{tree_entry: entry_ptr, owned: false} )
                }
            }
        }
    }

    /// Lookup a tree entry by SHA value.
    /// Warning: this must examine every entry in the tree, so it is not fast.
    fn entry_byoid(&self, oid: &OID) -> Option<~TreeEntry>
    {
        unsafe {
            let entry_ptr = ext::git_tree_entry_byoid(self.tree, oid);
            if entry_ptr == ptr::null() {
                None
            } else {
                Some( ~TreeEntry{tree_entry: entry_ptr, owned: false} )
            }
        }
    }

    /// Retrieve a tree entry contained in a tree or in any of its subtrees,
    /// given its relative path.
    fn entry_bypath(&self, path: &str) -> Option<~TreeEntry>
    {
        do str::as_c_str(path) |c_path| {
            unsafe {
                let mut entry_ptr:*ext::git_tree_entry = ptr::null();
                if ext::git_tree_entry_bypath(&mut entry_ptr, self.tree, c_path) == 0 {
                    Some( ~TreeEntry{tree_entry: entry_ptr, owned: true} )
                } else {
                    None
                }
            }
        }
    }

    /// Traverse the entries in a tree and its subtrees in pre order.
    ///
    /// Children subtrees will be automatically loaded as required, and the `callback` will be
    /// called once per entry with the current (relative) root for the entry and
    /// the entry data itself.
    ///
    /// If the callback returns WalkSkip, the passed entry will be skipped on the traversal.
    /// WalkPass continues the walk, and WalkStop stops the walk.
    ///
    /// The function returns false if the loop is stopped by StopWalk
    fn walk_preorder(&self, callback: &fn(&str, &TreeEntry) -> WalkMode) -> bool
    {
        unsafe {
            let fptr: *c_void = cast::transmute(&callback);
            let result = ext::git_tree_walk(self.tree, ext::GIT_TREEWALK_PRE, pre_walk_cb, fptr);
            if result == 0 {
                true
            } else if result == ext::GIT_EUSER {
                false
            } else {
                raise!(conditions::iter_fail::cond);
                false
            }
        }
    }

    /// Traverse the entries in a tree and its subtrees in post order.
    ///
    /// Children subtrees will be automatically loaded as required, and the `callback` will be
    /// called once per entry with the current (relative) root for the entry and
    /// the entry data itself.
    ///
    /// If the callback returns false, the loop stops
    ///
    /// The function returns false if the loop is stopped by callback
    fn walk_postorder(&self, callback: &fn(&str, &TreeEntry) -> bool) -> bool
    {
        unsafe {
            let fptr: *c_void = cast::transmute(&callback);
            let result = ext::git_tree_walk(self.tree, ext::GIT_TREEWALK_POST, post_walk_cb, fptr);
            if result == 0 {
                true
            } else if result == ext::GIT_EUSER {
                false
            } else {
                raise!(conditions::iter_fail::cond);
                false
            }
        }
    }
}

pub enum WalkMode {
    WalkSkip = 1,
    WalkPass = 0,
    WalkStop = -1,
}

extern fn pre_walk_cb(root: *c_char, entry: *ext::git_tree_entry, payload: *c_void) -> c_int
{
    unsafe {
        let op_ptr: *&fn(&str, &TreeEntry) -> WalkMode = cast::transmute(payload);
        let op: &fn(&str, &TreeEntry) -> WalkMode = *op_ptr;
        let root_str = str::raw::from_c_str(root);
        let entry = TreeEntry { tree_entry: entry, owned: false };
        op(root_str, &entry) as c_int
    }
}

extern fn post_walk_cb(root: *c_char, entry: *ext::git_tree_entry, payload: *c_void) -> c_int
{
    unsafe {
        let op_ptr: *&fn(&str, &TreeEntry) -> bool = cast::transmute(payload);
        let op: &fn(&str, &TreeEntry) -> bool = *op_ptr;
        let root_str = str::raw::from_c_str(root);
        let entry = TreeEntry { tree_entry: entry, owned: false };
        if op(root_str, &entry) {
            // continue
            0
        } else {
            // negative value stops the walk
            -1
        }
    }
}

impl BaseIter<TreeEntry> for Tree {
    /// traverse Tree with internal storage order
    fn each(&self, blk: &fn(v: &TreeEntry) -> bool) {
        unsafe {
            let size = ext::git_tree_entrycount(self.tree);
            let mut idx:size_t = 0;
            while idx < size {
                let entry_ptr = ext::git_tree_entry_byindex(self.tree, idx);
                if entry_ptr == ptr::null() {
                    fail!(~"bad entry pointer")
                }
                let entry = TreeEntry { tree_entry: entry_ptr, owned: false };
                if !blk(&entry) {
                    break
                }
            }
        }
    }

    fn size_hint(&self) -> Option<uint> {
        unsafe {
            Some(ext::git_tree_entrycount(self.tree) as uint)
        }
    }
}

#[unsafe_destructor]
impl Drop for Tree {
    fn finalize(&self) {
        unsafe {
            ext::git_tree_free(self.tree);
        }
    }
}

pub impl TreeEntry {
    /// Get the filename of a tree entry
    fn name(&self) -> ~str
    {
        unsafe {
            str::raw::from_c_str(ext::git_tree_entry_name(self.tree_entry))
        }
    }

    /// Get the id of the object pointed by the entry
    fn id(&self) -> &'self OID
    {
        unsafe {
            cast::transmute(ext::git_tree_entry_id(self.tree_entry))
        }
    }

    fn otype(&self) -> OType
    {
        unsafe {
            ext::git_tree_entry_type(self.tree_entry)
        }
    }

    fn filemode(&self) -> FileMode
    {
        unsafe {
            ext::git_tree_entry_filemode(self.tree_entry)
        }
    }
}

#[unsafe_destructor]
impl Drop for TreeEntry {
    fn finalize(&self) {
        unsafe {
            if self.owned {
                ext::git_tree_entry_free(self.tree_entry);
            }
        }
    }
}

impl Clone for TreeEntry {
    fn clone(&self) -> TreeEntry {
        unsafe {
            TreeEntry {
                tree_entry: ext::git_tree_entry_dup(self.tree_entry),
                owned: self.owned,
            }
        }
    }
}

impl TotalOrd for TreeEntry {
    fn cmp(&self, other: &TreeEntry) -> Ordering {
        unsafe {
            let comp = ext::git_tree_entry_cmp(self.tree_entry, other.tree_entry);
            if comp < 0 {
                Less
            } else if comp == 0 {
                Equal
            } else {
                Greater
            }
        }
    }
}

pub impl super::TreeBuilder {
    /// Create a new tree builder.
    /// The tree builder can be used to create or modify trees in memory and
    /// write them as tree objects to the database.
    /// The tree builder will start with no entries and will have to be filled manually.
    fn new() -> TreeBuilder
    {
        let mut bld:*ext::git_treebuilder = ptr::null();
        unsafe {
            if ext::git_treebuilder_create(&mut bld, ptr::null()) == 0 {
                super::TreeBuilder { bld: bld }
            } else {
                raise!(conditions::bad_treebuilder::cond)
            }
        }
    }

    /// Create a new tree builder.
    /// The tree builder will be initialized with the entries of the given tree.
    fn from_tree(tree: &Tree) -> TreeBuilder
    {
        let mut bld:*ext::git_treebuilder = ptr::null();
        unsafe {
            if ext::git_treebuilder_create(&mut bld, tree.tree) == 0 {
                super::TreeBuilder { bld: bld }
            } else {
                raise!(conditions::bad_treebuilder::cond)
            }
        }
    }

    /// Clear all the entires in the builder
    fn clear(&mut self)
    {
        unsafe {
            ext::git_treebuilder_clear(self.bld);
        }
    }

    /// Get an entry from the builder from its filename
    fn get(&self, filename: &str) -> ~TreeEntry
    {
        do str::as_c_str(filename) |c_filename| {
            unsafe {
                let entry_ptr = ext::git_treebuilder_get(self.bld, c_filename);
                ~TreeEntry { tree_entry: entry_ptr, owned: false }
            }
        }
    }

    /// Add or update an entry to the builder
    ///
    /// Insert a new entry for `filename` in the builder with the
    /// given attributes.
    ///
    /// If an entry named `filename` already exists, its attributes
    /// will be updated with the given ones.
    ///
    /// No attempt is being made to ensure that the provided oid points
    /// to an existing git object in the object database, nor that the
    /// attributes make sense regarding the type of the pointed at object.
    ///
    /// filename: Filename of the entry
    /// id: SHA1 OID of the entry
    /// filemode: Folder attributes of the entry. This parameter must not be GIT_FILEMODE_NEW
    fn insert(&mut self, filename: &str, id: &OID, filemode: FileMode) -> ~TreeEntry
    {
        do str::as_c_str(filename) |c_filename| {
            unsafe {
                let mut entry_ptr:*ext::git_tree_entry = ptr::null();
                if(ext::git_treebuilder_insert(&mut entry_ptr, self.bld, c_filename, id,
                                                filemode) == 0) {
                    ~TreeEntry { tree_entry: entry_ptr, owned: false }
                } else {
                    raise!(conditions::bad_treeentry::cond)
                }
            }
        }
    }

    /// Remove an entry from the builder by its filename
    /// return true if successful, false if the entry does not exist
    fn remove(&mut self, filename: &str) -> bool
    {
        do str::as_c_str(filename) |c_filename| {
            unsafe {
                ext::git_treebuilder_remove(self.bld, c_filename) == 0
            }
        }
    }

    /// Filter the entries in the tree
    ///
    /// The `filter` closure will be called for each entry in the tree with a
    /// ref to the entry;
    /// if the closure returns false, the entry will be filtered (removed from the builder).
    fn filter(&mut self, filter: &fn(&TreeEntry) -> bool)
    {
        unsafe {
            ext::git_treebuilder_filter(self.bld, filter_cb, cast::transmute(&filter));
        }
    }

    /// Write the contents of the tree builder as a tree object
    ///
    /// The tree builder will be written to the given `repo`, and its
    /// identifying SHA1 hash will be returned
    ///
    /// repo: Repository in which to store the object
    fn write(&mut self, repo: &mut Repository) -> OID
    {
        let mut oid = OID { id: [0, ..20] };
        unsafe {
            if ext::git_treebuilder_write(&mut oid, repo.repo, self.bld) == 0 {
                oid
            } else {
                raise!(conditions::bad_oid::cond)
            }
        }
    }

    /// Get the number of entries listed in a treebuilder
    fn entrycount(&self) -> uint
    {
        unsafe {
            ext::git_treebuilder_entrycount(self.bld) as uint
        }
    }
}

extern fn filter_cb(entry: *ext::git_tree_entry, payload: *c_void) -> c_int
{
    unsafe {
        let op_ptr: *&fn(&TreeEntry) -> bool = cast::transmute(payload);
        let op: &fn(&TreeEntry) -> bool = *op_ptr;
        let entry = TreeEntry { tree_entry: entry, owned: false };
        if op(&entry) {
            0
        } else {
            1
        }
    }
}

#[unsafe_destructor]
impl Drop for TreeBuilder {
    fn finalize(&self) {
        unsafe {
            ext::git_treebuilder_free(self.bld);
        }
    }
}
