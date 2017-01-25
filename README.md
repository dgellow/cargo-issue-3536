Reproduce the [cargo](https://github.com/rust-lang/cargo) issue https://github.com/rust-lang/cargo/issues/3536

## How to reproduce?

- Run `cargo clean`
- Run `cargo build`. For some reasons the first build fails instantly.
- Run `cargo build` again. The second build will fail when building `b` then run the file `deps/c/build.rs` that force the process to wait 5 seconds.
