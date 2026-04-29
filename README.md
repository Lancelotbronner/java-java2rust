# Java to Rust

This fork was rewritten from scratch.
The forked code serves only as a reminder for features to implement, it isn't called anywhere.

## Features

- Converts Java names to Rust
- Converts classes, records and their local variants to structs
  - Parameters either take a primitive by value or a Java class by reference
  - Promotes to `const` when possible
  - Methods take `&mut self` if fields are modified (directly or indirectly) and `&self` otherwise
  - *analysis for propagating all thrown exception using a call graph, currently no generated code*
  - *currently no analysis for `mut` parameters*
  - *currently no analysis for `&mut` parameter types*
- Converts statements and expressions to Rust
  - *some support for simple `try-catch` statements*
  - *currently no analysis for `mut` locals*
  - Automatic `self.`
- Support for specifying maven dependencies in order to improve analysis

## Usage

There's a CLI tool available with embedded help.

## Contributing

Feel free to fork and open a PR!
