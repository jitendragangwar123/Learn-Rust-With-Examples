### Learn-Rust-With-Examples
This repository comprises concepts and examples related to the Rust programming language. It provides a resource for understanding and practicing `Rust` language concepts through practical examples.

### Resources 
- [Rust-By-Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust-By-Practice](https://practice.rs/why-exercise.html)
- [The-Rust-Programming-Language](https://doc.rust-lang.org/book/title-page.html)
- [Rust-Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
- [Roadmap](https://roadmap.sh/rust)

### Table Of Contents
- [Introduction](https://github.com/jitendragangwar123/Learn-Rust-With-Examples/tree/main/Introduction)
- [Primitives]()
- [Custom Types]()



  
#### For MacOS:
```shell
// Installation
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  Rust is installed now. Great!
$ rustc –-version // check the version
$ rustup update  // updating 
$ rustup self uninstall // uninstalling

// Create New Project
$ mkdir ~/RustWithExamples
$ cd ~/RustWithExamples
$ touch hello_world.rs // create rust file
$ rustc hello_world.rs // to compile the file
$ ./hello_world // to run the file
  Hello, world!  // output
```

#### For Linux:
```shell
// Installation
$ sudo apt-get install rustc cargo

// Create New Project
$ cargo new my_project
$ cargo build //to build the project means compiles the project, and generates an executable file in the target/debug directory
$ cargo run // to run the executable file
$ cargo check // to check the syntax and type-checking

`Cargo.toml`: Store metadata about your project
    [package]
    name = "my_project"
    version = "0.1.0"
    authors = ["Your Name <your@email.com>"]
    [dependencies]
    rand = "0.8.4"
```
