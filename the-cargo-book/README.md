
https://doc.rust-lang.org/cargo/index.html

# Basic Cargo Directory Structure
```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

- Cargo.toml and Cargo.lock are stored in the root of your package (package root).
- Source code goes in the src directory.
- The default library file is src/lib.rs.
- The default executable file is src/main.rs.
  - Other executables can be placed in src/bin/.
- Benchmarks go in the benches directory.
- Examples go in the examples directory.
- Integration tests go in the tests directory.

Keep `Cargo.lock` in binary projects, but out of library projects. (https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries)

Cargo looks for tests to run in two places: in each of your src files and any tests in tests/. Tests in your src files should be unit tests, and tests in tests/ should be integration-style tests.

Platform specific dependencies:
```
[target.'cfg(windows)'.dependencies]
winhttp = "0.4.0"

[target.'cfg(target_arch = "x86")'.dependencies]
native = { path = "native/i686" }
```

Using `mod`:
```rust
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```
We’re still declaring the client module here, but by replacing the block with a semicolon, we’re telling Rust to look in another location for the code defined within the scope of the client module. In other words, the line mod client; means this:
```rust
mod client {
    // contents of client.rs
}
```

If you do a `mod module_name`, cargo will look in the folder for `module_name.rs` or `module_name/mod.rs`.

# Workspaces in Cargo.toml
The key points of workspaces (see the [workspace] tag) are:
- All packages share a common Cargo.lock file which resides in the workspace root.
- All packages share a common output directory, which defaults to a directory named target in the workspace root.
- The [patch], [replace] and [profile.*] sections in Cargo.toml are only recognized in the root manifest, and ignored in member crates' manifests.

When inside a subdirectory within the workspace, Cargo will automatically search the parent directories for a Cargo.toml file with a [workspace] definition to determine which workspace to use.

# Build scripts
Placing a file named build.rs in the root of a package will cause Cargo to compile that script and execute it just before building the package. Can add dependencies for this script in `[build-dependencies]`.

# Publishing to crates.io
Check https://doc.rust-lang.org/cargo/reference/publishing.html