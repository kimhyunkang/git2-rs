git2-rs
=======

libgit2 bindings for Rust

## libgit2 Compatibility

git2-rs compiles against libgit2 0.18.0

## Thread Safety

If you need to call library functions from multiple tasks, you have to initialize and de-initialize the library

You can initialize the library with calling ```git2::threads_init()``` before any other functions. De-initialization is done with calling ```git2::threads_shutdown()```
