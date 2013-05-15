#[link(name = "git2",
       vers = "0.1-pre",
       url = "https://github.com/kimhyunkang/git2-rs")];

#[comment = "libgit2 binding for Rust"];
#[license = "MIT"];

#[crate_type = "lib"];

pub mod error;
pub mod ext;
pub mod types;
pub mod repository;

pub type Repository = types::Repository;
pub type GitError = types::GitError;
