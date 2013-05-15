use ext;

pub struct GitError {
    pub message: ~str,
    pub klass: ext::git_error_t,
}

pub struct Repository {
    pub repo: *ext::git_repository,
}
