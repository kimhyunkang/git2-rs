use core::libc::size_t;
use super::*;
use ext;

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
}

impl BaseIter<TreeEntry> for Tree {
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
