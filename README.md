### Learn-Rust-With-Examples
This repository comprises concepts and examples related to the Rust programming language. It provides a resource for understanding and practicing `Rust` language concepts through practical examples.

#### Resources 
- [Rust-By-Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust-By-Practice](https://practice.rs/why-exercise.html)
- [The-Rust-Programming-Language](https://doc.rust-lang.org/book/title-page.html)
- [Rust-Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

### Table Of Contents
- [Introduction](https://github.com/jitendragangwar123/Learn-Rust-With-Examples/tree/main/Introduction)
- [Primitives]()
- [Custom Types]()



  
#### Installation
##### For MacOS:
```shell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  Rust is installed now. Great!
$ rustc –-version
```
###### Updating and Uninstalling
```shell
$ rustup update
$ rustup self uninstall
```
###### Create New Project
```shell
$ mkdir ~/RustWithExamples
$ cd ~/RustWithExamples
$ touch hello_world.rs
```
```shell
$ rustc hello_world.rs // to compile the file
$ ./hello_world // to run the file
Hello, world!  // output
```
##### For Linux:
```shell
$ sudo apt-get install rustc cargo
```
###### Create New Project
```shell
$ cargo new my_project
```
```shell
$ cargo build //to build the project means compiles the project, and generates an executable file in the target/debug directory
$ cargo run // to run the executable file
$ cargo check // to check the syntax and type-checking
```
```shell
Cargo.toml:- store metadata about your project
```

