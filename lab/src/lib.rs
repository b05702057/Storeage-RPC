//! ## CSE 223B Labs
//!
//! Welcome to CSE 223B. Below you'll find some information on setting up an
//! environment to complete the labs for the course. Instructions for each lab
//! can be found at
//!
//! - [Lab 1](lab1)
//! - [Lab 2](lab2)
//! - [Lab 3](lab3)
//!
//! ## Programming Language
//!
//! You will write the labs in the [Rust](https://www.rust-lang.org/)
//! programming language. It is a community-developed language originally
//! started at Mozilla. Rust is a low-level language which eschews any kind of
//! managed runtime like you might find in Java, Python, or Go. Rust's main
//! selling point is it's **compile-time** [borrow
//! checker](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
//! that prevents invalid memory accesses at runtime.
//!
//! Here is some key documentation on the Rust programming language:
//!
//! - ["The Book"](https://doc.rust-lang.org/book/title-page.html)
//! - [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
//! - [Learn Rust Page](https://www.rust-lang.org/learn)
//! - [The Rust Playground](https://play.rust-lang.org/)
//!
//! You should be able to find a lot of documents about the Rust language on the
//! web, especially from the official site. **[We highly recommend reading "The
//! Book"](https://doc.rust-lang.org/book/title-page.html)** and following parts
//! of the [Rustlings](https://github.com/rust-lang/rustlings/) course.
//!
//! **Rust takes time to understand and has a steep learning curve**. In the
//! early stages of using the language, getting your program just to compile
//! might take you 30 minutes or more. **Do not underestimate the amount of time
//! you will need** to spend working on these assignments. Start early, **[read
//! the book](https://doc.rust-lang.org/book/title-page.html)**, and search on
//! the web if you have problems compiling your program. **Read the compiler
//! error messages too**. The Rust compiler has some of the most helpful error
//! messages of all modern languages. If that doesn't help, try **[reading the
//! book again](https://doc.rust-lang.org/book/title-page.html)**. Finally, if you
//! still can't figure out your issue ask for help on piazza or come to office
//! hours if you truly feel stuck.
//!
//! **[READ THE BOOK](https://doc.rust-lang.org/book/title-page.html)**
//!
//! ---
//!
//! ### Install Rust
//!
//! To get started with the Rust language, we recommend using the official
//! [`rustup`](http://rustup.sh/) tool. This will install the latest stable Rust
//! toolchain along with a number of programs useful for building applications
//! with the language.
//!
//! #### Cargo
//!
//! The most important tool you'll need from the toolchain is `cargo`. `cargo`
//! is a user-friendly wrapper around the rust compiler (rustc) and handles
//! downloading, compiling, testing, formatting, checking, linting, documenting
//! Rust code, and more.
//!
//! Cargo looks for a manifest file, `Cargo.toml` in your current directory to
//! figure out exactly which files it needs to build, test, or run. The file can
//! define dependencies, build options, metadata and many other options. You
//! shouldn't need to modify any `Cargo.toml` files for these assignments.
//!
//! Here are some of the most useful cargo commands
//!
//! - `cargo build` will compile your code with no optimization
//!     - `cargo build --release` compiles with optimizations (e.g. similar to
//!       `gcc -O2`)
//!     - all compilation output can be found in the `./target` directory
//! - `cargo test` runs all of the tests defined in the projects
//!     - `cargo test --tests` runs integration tests
//!     - `cargo test -p <package>` runs all of the tests in `<package>`
//! - `cargo fmt` will format your code according to standard Rust formatting
//!   guidelines
//! - `cargo check` looks for compilation errors in packages and dependencies.
//!   It basically performs compilation without the final step of code
//!   generation. It is faster than `cargo build` for checking if your code
//!   compiles.
//! - `cargo run` can run binaries defined in `Cargo.toml`
//!     - `cargo run --bin <bin-name>` will run the `<bin-name>` binary
//!     - pass arguments to the binary with `cargo run --bin <bin-name> --
//!       <arg1> <arg2> ...`
//!
//! #### Clippy
//!
//! Rust also has a non-default code linting tool called Clippy. It can help
//! point out simple optimizations and make your code more idiomatic. Clippy is
//! entirely optional to use, but is immensely helpful in identifying use of
//! anti-patterns in your code.
//!
//! - To install clippy: `rustup component add clippy`, then try running with
//!   `cargo clippy`
//!
//! #### Basic Rust Concepts
//!
//! It is difficult to cover all of Rust's great (and not-so great) features. We
//! recommend thoroughly reading through ["The
//! Book"](https://doc.rust-lang.org/book/title-page.html) or [Rust By
//! Example](https://doc.rust-lang.org/rust-by-example/) to learn the ins and
//! outs of the language. Below we try to highlight some of the most important
//! concepts you should understand.
//!
//! - Rust code is organized into *crates* and *modules*
//!   - a set of modules make up a crate.
//! - Compile, run, and format code with `cargo` (`build`, `run`, `fmt`)
//! - Once cargo is installed, view documentation for the crate and dependencies
//!   with `cargo doc --open`
//! - In Rust, variables must be explicitly defined as mutable using the `mut`
//!   keyword.
//! - References to variables in Rust can be *owned* or *borrowed*.
//! - `Vec<T>` is the standard type used to represent a series of items, similar
//!   to C++. Array types (`[T; usize]`) are rarely used. Usually you should opt
//!   for a slice (`&[T]`)
//! - `std::collection::HashMap` or `std::collections::BTreeMap` can be used to
//!   store a set of key-value pairs.
//! - The `Result<T, E>` is the standard return type used for functions which
//!   might return errors. When calling a function which returns a result, use
//!   the `?` operator after the `()` to unwrap the result to type `T`
//! - some built-in macros that you might find useful are `unimplemented!()`,
//!   `todo!()`, `println!()`, `print!()`, and `format!()`.
//! - you can write a loop using the `loop` keyword and run a function over an
//!   iterator with `for x in a.iter()`
//! - values are returned from functions by simply omitting the trailing
//!   semicolon `;` at the end of a statement.
//! - Store items on the heap using the `Box<T>` type.
//! - You can write interfaces in Rust using the `trait` keyword, and then
//! implement a trait for a specific struct with `impl <Trait> for <Struct> {
//! ... }`
//!
//! ## The Tribbler Story
//!
//! Some cowboy programmer wrote a simple online microblogging service called
//! Tribbler and, leveraging the power of the Web, it becomes quite popular.
//! However, the program runs in a single process on a single machine; it does
//! not scale, cannot support many concurrent connections, and is vulnerable to
//! machine crashes. Knowing that you are taking the distributed computing
//! system course at UCSD, he asks you for help. You answered his call and are
//! starting work on this project.
//!
//! Your goal is to refactor Tribbler into a distributed system, making it more
//! robust and scalable.
//!
//! ## Getting Started
//!
//! We will be using Github Classroom to distribute assignments and manage
//! submissions. We will send out invitation links for each individual lab
//! through Piazza/Canvas. Once you access these links, a GitHub repository
//! containing Tribbler starter code will be created for you. It comes with
//! necessary instructions on how to setup environment before you start, and
//! submit your work when you are done.
//!
//! Once you download your first lab (and before you start working on it), you
//! can run the basic version of Tribbler. The Tribbler project is written in
//! Rust. To get started, run these commands from the command line:
//!
//! You can do some basic testing to see if the framework is in good shape:
//!
//! ```console
//! $ cargo test -p tribbler
//! ```
//!
//! The basic Tribbler service should now be installed on the system from your
//! home directory. Let's give it a try:
//!
//! ```console
//! $ cargo run --bin trib-front
//! ```
//!
//! The program should show the URL it is running under.
//!
//! Open your browser and type in the address: `http://<host-name>:<port>`. For
//! example, if you are using your local machine, and Tribbler is running on
//! port 8080, then open `http://localhost:8080`. If you are using AWS EC2
//! machine, you can use its public DNS name as host name (make sure to allow
//! traffic on this port to your instance). You should see a list of Tribbler
//! users. You can view their tribs and login as them (with no authentication).
//!
//! This is how Tribbler looks to users.  It is a single web page that performs
//! AJAX calls (a type of web-based RPC) to the back-end web server. The
//! webserver then in turn calls the Tribbler logic functions and returns the
//! results back to the Web page in the browser.
//!
//! ## Source Code Organization
//!
//! The source code in the `trib` package repository is organized as follows:
//!
//! - `lab` has the skeleton code for completing the class assignments
//! - [tribbler] defines the common Tribbler interfaces and data structures.
//! - [tribbler::ref_impl] is a reference monolithic implementation of the
//!   [Server](tribbler::trib::Server) interface. All the server logic runs in
//!   one single process. It is not scalable and is vulnerable to machine
//!   crashes.
//! - [tribbler::storage] contains an in-memory thread-safe implementation of
//!   the [tribbler::storage::Storage] interface. We will use this as the basic
//!   building block for our back-end storage system.
//! - [tribbler::addr] provides helper functions that check if an address
//!   belongs to the machine that the program is running.
//! - [tribbler::addr::rand] provides helper functions that generate a network
//!   address with a random port number.
//! - [tribbler::colon] provides helper functions that escape and unescape
//!   colons in a string.
//! - `lab/tests` provides several basic test cases for the interfaces.
//! - `cmd/src/trib_front.rs` is the web-server launcher that you run.
//! - `cmd/src/kv_client.rs` is a command line key-value RPC client for quick
//!   testing.
//! - `cmd/src/kv_server.rs` runs a key-value service as an RPC server.
//! - `cmd/src/bins_client.rs` is a bin storage service client.
//! - `cmd/src/bins_back.rs` is a bin storage service back-end launcher.
//! - `cmd/src/bins_keep.rs` is a bin storage service keeper launcher.
//! - `cmd/src/bins_mkcfg.rs` generates a bin storage configuration file.
//! - `www/` contains the static files (html, css, js, etc.) for the web
//!   front-end.
//!
//! **Don't be scared by the number of modules**. Most of the modules are very
//! small, and you don't have to interact with all of them at once. All Rust
//! language files under the `tribbler` directory are less than 2500 lines in
//! total, so these packages aren't huge and intimidating.
//!
//! Throughout the entire lab, you do not need to (and should not) modify
//! anything in the [tribbler] crate. If you feel that you have to change some
//! code to complete your lab, please first discuss it with the TA. You are
//! always welcome to read the code in the tribbler crate or read its
//! [documentation][tribbler]. If you find a bug and report it, you might get
//! some bonus credit.
//!
//! ## Your Job
//!
//! Your job is to complete the implementation of the `lab` package.
//!
//! It would be good practice for you to periodically commit your code into your
//! git repo.
//!
//! ## Lab Roadmap
//!
//! - [Lab 1](crate::lab1). Wrap the key-value storage service with RPC so that
//!   a remote client can store data remotely.
//! - [Lab 2](crate::lab2). Reimplement the Tribbler service, splitting the
//!   current Tribbler logic into stateless scalable front-ends and scalable
//!   key-value store back-ends. The front-ends will call the back-ends via the
//!   RPC mechanism implemented in Lab 1. When this lab is done, you will have
//!   made both the front-end and the back-end scalable.
//! - [Lab 3](crate::lab3). We make the back-ends fault-tolerent with
//!   replication and by using techniques like distributed hash tables. At the
//!   end of this lab, back-end servers can join, leave, or be killed, without
//!   affecting the service.
//!
//! By the end of the labs, you will have an implementation of Tribbler that is
//! scalable and fault-tolerant.
//!
//! ## Misc
//!
//! You can write your code on your own machine if you want to. See `rustup`
//! language's [install](http://rustup.sh/) page for more information on how to
//! install in different environments
//!
//! Visual Studio Code (VSCode) has SSH extensions to help with remote
//! development and the
//! [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer),
//! Rusts's community supported language server, which can make editing rust
//! code easier. If you want to use a remote developement environment on AWS
//! with rust-analyzer, you might need to upgrade to a larger instance type
//! (e.g. a t3.medium) to accomodate the larger memory footprint of running
//! rust-analyzer.
//!
//! ## Ready?
//!
//! If you feel comfortable with the lab setup, continue on to [Lab 1](lab1).
//!
mod keeper;
pub mod lab1;
pub mod lab2;
pub mod lab3;
