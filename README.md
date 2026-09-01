# Java to Rust

This fork was rewritten from scratch.
The forked code serves only as a reminder for features to implement, it isn't called anywhere.

## Features

- Converts Java names to Rust
  - *currently no handling for reserved names*
  - *currently no overload handling*
- Converts classes, records and their local variants to structs
  - Parameters either take a primitive by value or a Java class by reference
  - Promotes to `const` when possible
  - Methods take `&mut self` if fields are modified (directly or indirectly) and `&self` otherwise
  - *analysis for propagating all thrown exception using a call graph, currently no generated code*
  - *currently no analysis for `mut` parameters*
  - *currently no analysis for `&mut` parameter types*
- Converts statements and expressions to Rust
  - Automatic `self.` for field access
  - *some support for simple `try-catch` statements*
  - *currently no analysis for `mut` locals*
  - *currently no distinction between `>>` and `>>>`*
- Support for specifying maven dependencies in order to improve analysis

## Progress

You can see the current state of things in the `generated` folder which is the transpiled project itself.
I generate it with the following command:

```sh
java2rust generated --sources . --maven com.github.javaparser:javaparser-core:3.28.0 --maven org.apache.commons:commons-lang3:3.20.0
```

## Roadmap

- More tests for what is supposed to work right now, ensure they all pass
- Assign `RustPackage` to their `RustJar`, allowing imports to track the crate
- Generate a `Cargo.toml` workspace containing all generated crates
- Ensure all printed types use the full path
- Generate a `stderr.log` alongside the generated files for any occuring errors
- Generate a `rustc.log` which documents the build errors

## Usage

There's a CLI tool available with embedded help.

## Contributing

Feel free to fork and open a PR!
