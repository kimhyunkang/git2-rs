use std::libc::c_int;
use std::{ptr, cast};
use std::str::raw::from_c_str;
use super::{OID, OType, FileMode, GitError};
use super::{git_error, last_error};
use super::Repository;
use ext;

pub struct Tree<'r> {
    // TODO: make this field priv
    tree: *ext::git_tree,
    priv owner: &'r Repository,
}

impl<'r> Tree<'r> {
    pub fn new(tree: *ext::git_tree, owner: &'r Repository) -> Tree<'r>
    {
        Tree {
            tree: tree,
            owner: owner
        }
    }

    /// Get the id of a tree.
    pub fn id<'r>(& self) -> &'r OID
    {
        unsafe {
            cast::transmute(ext::git_tree_id(self.tree))
        }
    }

    /// Lookup a tree entry by its filename
    pub fn entry_byname(&self, filename: &str) -> Option<~TreeEntry>
    {
        filename.with_c_str(|c_filename| {
            unsafe {
                let entry_ptr = ext::git_tree_entry_byname(self.tree, c_filename);
                if entry_ptr == ptr::null() {
                    None
                } else {
                    Some( ~TreeEntry::new(entry_ptr, false) )
                }
            }
        })
    }

    /// Lookup a tree entry by SHA value.
    /// Warning: this must examine every entry in the tree, so it is not fast.
    pub fn entry_byoid(&self, oid: &OID) -> Option<~TreeEntry>
    {
        unsafe {
            let entry_ptr = ext::git_tree_entry_byoid(self.tree, oid);
            if entry_ptr == ptr::null() {
                None
            } else {
                Some( ~TreeEntry::new(entry_ptr, false) )
            }
        }
    }

    /// Retrieve a tree entry contained in a tree or in any of its subtrees,
    /// given its relative path.
    pub fn entry_bypath(&self, path: &str) -> Option<~TreeEntry>
    {
        path.with_c_str(|c_path| {
            unsafe {
                let mut entry_ptr:*ext::git_tree_entry = ptr::null();
                if ext::git_tree_entry_bypath(&mut entry_ptr, self.tree, c_path) == 0 {
                    Some( ~TreeEntry::new(entry_ptr, true) )
                } else {
                    None
                }
            }
        })
    }
}

#[unsafe_destructor]
impl<'r> Drop for Tree<'r> {
    fn drop(&mut self) {
        unsafe {
            ext::git_tree_free(self.tree);
        }
    }
}

pub struct TreeEntry {
    priv tree_entry: *ext::git_tree_entry,
    priv owned: bool,
}

impl TreeEntry {
    fn new(tree_entry: *ext::git_tree_entry, owned: bool) -> TreeEntry {
        TreeEntry {
            tree_entry: tree_entry,
            owned: owned
        }
    }

    /// Get the filename of a tree entry
    pub fn name(&self) -> ~str
    {
        unsafe {
            from_c_str(ext::git_tree_entry_name(self.tree_entry))
        }
    }

    /// Get the id of the object pointed by the entry
    pub fn id<'r>(&self) -> &'r OID
    {
        unsafe {
            cast::transmute(ext::git_tree_entry_id(self.tree_entry))
        }
    }

    pub fn otype(&self) -> OType
    {
        unsafe {
            ext::git_tree_entry_type(self.tree_entry)
        }
    }

    pub fn filemode(&self) -> FileMode
    {
        unsafe {
            ext::git_tree_entry_filemode(self.tree_entry)
        }
    }
}

#[unsafe_destructor]
impl Drop for TreeEntry {
    fn drop(&mut self) {
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
            TreeEntry::new(ext::git_tree_entry_dup(self.tree_entry), self.owned)
        }
    }
}

#[inline]
fn tree_entry_cmp(a: &TreeEntry, b: &TreeEntry) -> c_int
{
    unsafe {
        ext::git_tree_entry_cmp(a.tree_entry, b.tree_entry)
    }
}

impl Eq for TreeEntry {
    fn eq(&self, other: &TreeEntry) -> bool {
        tree_entry_cmp(self, other) == 0
    }

    fn ne(&self, other: &TreeEntry) -> bool {
        tree_entry_cmp(self, other) != 0
    }
}

impl Ord for TreeEntry {
    fn lt(&self, other: &TreeEntry) -> bool {
        tree_entry_cmp(self, other) < 0
    }
    fn le(&self, other: &TreeEntry) -> bool {
        tree_entry_cmp(self, other) <= 0
    }
    fn gt(&self, other: &TreeEntry) -> bool {
        tree_entry_cmp(self, other) > 0
    }
    fn ge(&self, other: &TreeEntry) -> bool {
        tree_entry_cmp(self, other) >= 0
    }
}

impl TotalEq for TreeEntry {
    fn equals(&self, other: &TreeEntry) -> bool {
        tree_entry_cmp(self, other) == 0
    }
}

impl TotalOrd for TreeEntry {
    fn cmp(&self, other: &TreeEntry) -> Ordering {
        let comp = tree_entry_cmp(self, other);
        if comp < 0 {
            Less
        } else if comp == 0 {
            Equal
        } else {
            Greater
        }
    }
}

impl TreeBuilder {
    /// Clear all the entires in the builder
    pub fn clear(&self)
    {
        unsafe {
            ext::git_treebuilder_clear(self.bld);
        }
    }

    /// Get an entry from the builder from its filename
    pub fn get(&self, filename: &str) -> ~TreeEntry
    {
        filename.with_c_str(|c_filename| {
            unsafe {
                let entry_ptr = ext::git_treebuilder_get(self.bld, c_filename);
                ~TreeEntry::new(entry_ptr, false)
            }
        })
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
    pub fn insert(&self, filename: &str, id: &OID, filemode: FileMode) ->
        Result<~TreeEntry, (~str, GitError)>
    {
        filename.with_c_str(|c_filename| {
            unsafe {
                let mut entry_ptr:*ext::git_tree_entry = ptr::null();
                if(ext::git_treebuilder_insert(&mut entry_ptr, self.bld, c_filename, id,
                                                filemode) == 0) {
                    Ok( ~TreeEntry::new(entry_ptr, false) )
                } else {
                    Err( last_error() )
                }
            }
        })
    }

    /// Remove an entry from the builder by its filename
    /// return true if successful, false if the entry does not exist
    pub fn remove(&self, filename: &str) -> bool
    {
        filename.with_c_str(|c_filename| {
            unsafe {
                ext::git_treebuilder_remove(self.bld, c_filename) == 0
            }
        })
    }

    /// Write the contents of the tree builder as a tree object
    ///
    /// The tree builder will be written to the given `repo`, and its
    /// identifying SHA1 hash will be returned
    ///
    /// repo: Repository in which to store the object
    pub fn write(&self, repo: &Repository) -> OID
    {
        let mut oid = OID { id: [0, ..20] };
        unsafe {
            if ext::git_treebuilder_write(&mut oid, repo.repo, self.bld) != 0 {
                git_error::cond.raise(last_error())
            }
        }
        return oid;
    }

    /// Get the number of entries listed in a treebuilder
    pub fn entrycount(&self) -> uint
    {
        unsafe {
            ext::git_treebuilder_entrycount(self.bld) as uint
        }
    }
}

pub struct TreeBuilder {
    priv bld: *ext::git_treebuilder,
}

impl TreeBuilder {
    /// Create a new tree builder.
    /// The tree builder can be used to create or modify trees in memory and
    /// write them as tree objects to the database.
    /// The tree builder will start with no entries and will have to be filled manually.
    pub fn new() -> TreeBuilder
    {
        let mut bld:*ext::git_treebuilder = ptr::null();
        unsafe {
            if ext::git_treebuilder_create(&mut bld, ptr::null()) == 0 {
                TreeBuilder { bld: bld }
            } else {
                fail!(~"failed to create treebuilder")
            }
        }
    }

    /// Create a new tree builder.
    /// The tree builder will be initialized with the entries of the given tree.
    pub fn from_tree(tree: &Tree) -> TreeBuilder
    {
        let mut bld:*ext::git_treebuilder = ptr::null();
        unsafe {
            if ext::git_treebuilder_create(&mut bld, tree.tree) == 0 {
                TreeBuilder { bld: bld }
            } else {
                fail!(~"failed to create treebuilder")
            }
        }
    }
}


#[unsafe_destructor]
impl Drop for TreeBuilder {
    fn drop(&mut self) {
        unsafe {
            ext::git_treebuilder_free(self.bld);
        }
    }
}
