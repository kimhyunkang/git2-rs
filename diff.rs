use ext;
use super::{OID, DiffList};

pub enum DiffFlag {
    /** Reverse the sides of the diff */
    GIT_DIFF_REVERSE = (1 << 0),
    /** Treat all files as text, disabling binary attributes & detection */
    GIT_DIFF_FORCE_TEXT = (1 << 1),
    /** Ignore all whitespace */
    GIT_DIFF_IGNORE_WHITESPACE = (1 << 2),
    /** Ignore changes in amount of whitespace */
    GIT_DIFF_IGNORE_WHITESPACE_CHANGE = (1 << 3),
    /** Ignore whitespace at end of line */
    GIT_DIFF_IGNORE_WHITESPACE_EOL = (1 << 4),
    /** Exclude submodules from the diff completely */
    GIT_DIFF_IGNORE_SUBMODULES = (1 << 5),
    /** Use the "patience diff" algorithm (currently unimplemented) */
    GIT_DIFF_PATIENCE = (1 << 6),
    /** Include ignored files in the diff list */
    GIT_DIFF_INCLUDE_IGNORED = (1 << 7),
    /** Include untracked files in the diff list */
    GIT_DIFF_INCLUDE_UNTRACKED = (1 << 8),
    /** Include unmodified files in the diff list */
    GIT_DIFF_INCLUDE_UNMODIFIED = (1 << 9),

    /** Even with GIT_DIFF_INCLUDE_UNTRACKED, an entire untracked directory
     *  will be marked with only a single entry in the diff list; this flag
     *  adds all files under the directory as UNTRACKED entries, too.
     */
    GIT_DIFF_RECURSE_UNTRACKED_DIRS = (1 << 10),

    /** If the pathspec is set in the diff options, this flags means to
     *  apply it as an exact match instead of as an fnmatch pattern.
     */
    GIT_DIFF_DISABLE_PATHSPEC_MATCH = (1 << 11),

    /** Use case insensitive filename comparisons */
    GIT_DIFF_DELTAS_ARE_ICASE = (1 << 12),

    /** When generating patch text, include the content of untracked files */
    GIT_DIFF_INCLUDE_UNTRACKED_CONTENT = (1 << 13),

    /** Disable updating of the `binary` flag in delta records.  This is
     *  useful when iterating over a diff if you don't need hunk and data
     *  callbacks and want to avoid having to load file completely.
     */
    GIT_DIFF_SKIP_BINARY_CHECK = (1 << 14),

    /** Normally, a type change between files will be converted into a
     *  DELETED record for the old and an ADDED record for the new; this
     *  options enabled the generation of TYPECHANGE delta records.
     */
    GIT_DIFF_INCLUDE_TYPECHANGE = (1 << 15),

    /** Even with GIT_DIFF_INCLUDE_TYPECHANGE, blob->tree changes still
     *  generally show as a DELETED blob.  This flag tries to correctly
     *  label blob->tree transitions as TYPECHANGE records with new_file's
     *  mode set to tree.  Note: the tree SHA will not be available.
     */
    GIT_DIFF_INCLUDE_TYPECHANGE_TREES  = (1 << 16),

    /** Ignore file mode changes */
    GIT_DIFF_IGNORE_FILEMODE = (1 << 17),

    /** Even with GIT_DIFF_INCLUDE_IGNORED, an entire ignored directory
     *  will be marked with only a single entry in the diff list; this flag
     *  adds all files under the directory as IGNORED entries, too.
     */
    GIT_DIFF_RECURSE_IGNORED_DIRS = (1 << 18),

    /** Core Git scans inside untracked directories, labeling them IGNORED
     *  if they are empty or only contain ignored files; a directory is
     *  consider UNTRACKED only if it has an actual untracked file in it.
     *  This scan is extra work for a case you often don't care about.  This
     *  flag makes libgit2 immediately label an untracked directory as
     *  UNTRACKED without looking insde it (which differs from core Git).
     *  Of course, ignore rules are still checked for the directory itself.
     */
    GIT_DIFF_FAST_UNTRACKED_DIRS = (1 << 19),
}

pub struct DiffOption {
    flags: ~[DiffFlag],
    context_lines: u16,
    interhunk_lines: u16,
    old_prefix: ~str,
    new_prefix: ~str,
    pathspec: ~[~str],
    max_size: i64,
}

impl DiffOption {
    pub fn new() -> DiffOption {
        DiffOption {
            flags: ~[],
            context_lines: 3,
            interhunk_lines: 0,
            old_prefix: ~"a",
            new_prefix: ~"b",
            pathspec: ~[],
            max_size: 1 << 29,
        }
    }
}

pub struct DiffFile {
    oid: OID,
    path: ~str,
    size: i64,
    flags: u32,
    mode: u16,
}

#[unsafe_destructor]
impl Drop for DiffList {
    fn drop(&mut self) {
        unsafe {
            ext::git_diff_list_free(self.difflist);
        }
    }
}
