# libRustLexbor-sys

It is rust bindings and wrapper around [Lexbor](https://github.com/lexbor/lexbor) an open source HTML Renderer library. Create contains the Lexbor translated headers to use this library in Rust programs.



### Table of contents

* [Requierements](#requirements)
* [Installation](#installation)
* [Usage](#usage)
* [Bindings](#bindings)



### Requirements

* [Rust Compiler](https://www.rust-lang.org/)
* [Cargo package manager](https://www.rust-lang.org/)

Library is writing used latest stable Rust Compiler (rustc 1.46.0 (04488afe3 2020-08-24)).



### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
librustlexbor-sys = "0.1.*"
```



### Usage

Add to your crate root:

```rust
extern crate librustlexbor_sys;
```



### Bindings

[librustlexbor-sys](https://github.com/isemenkov/librustlexbor/tree/master/librustlexbor-sys) crate contains the lexbor translated headers to use this library in Rust programs.