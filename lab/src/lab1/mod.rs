#![allow(unused_variables)]
//! Welcome to Lab 1! The goal of this lab is to implement a key-value storage
//! service that is called via RPCs
//!
//! ## Lab 1
//!
//! Welcome to Lab 1. The goal of this lab is to implement a key-value storage
//! service that is called via RPCs. In particular you need to:
//!
//! 1. Implement a key-value storage server type that satisfies the
//!    [tribbler::storage::Storage] trait and takes http RPC requests from the
//!    network.
//! 2. Implement a key-value storage client type that satisfies the
//!    [tribbler::storage::Storage] trait which relays all of its requests back
//!    to the server.
//!
//! More specifically, you need to implement two entry functions that are
//! defined in the `lab/src/lab1/lab.rs` file: [serve_back] and [new_client].
//! Presently, they are both implemented with [todo!].
//!
//! ## Get Your Repo
//!
//! As mentioned earlier, we will send out an invitation link on Piazza/Canvas.
//! Once available, access it and follow the instructions provided in the
//! repository to get started.
//!
//! ## AWS VMs
//!
//! Distributed systems are by nature distributed, and to learn how to construct
//! them you will write code that runs on more than just your local machine. We
//! will be using Amazon Web Services (AWS) as a platform to build our
//! distributed systems; both because it allows us to deploy globally across
//! over 50 data centers, and because it is emblematic of most datacenters
//! today.
//!
//! To build distributed systems on AWS we first need to obtain compute
//! resources from it. We will be running our code in virtual machines (VMs). To
//! get a virtual machine on AWS follow the short tutorial supplied in your lab
//! 1 starter code `ec2-setup.md`. This tutorial will have you make an account
//! on AWS using your UCSD credentials. Virtual machines cost real money to run,
//! and their pricing is determined by how many resources are given to the
//! virual machine. You are not going to need high powered computation for these
//! assignments so stick to small virtual machines like t3.micro which only
//! costs about $.50 a day to run. Stopping a VM on the AWS web console will
//! stop the VM from charging your account money. Every student will get $50
//! worth of AWS credits, so if you launch a lot of VMs for testing make sure to
//! turn them off when you are done.
//!
//! Once you have a virtual machine provisioned try to ssh into it from your own
//! machine. Use the public IPv4 DNS address to ssh using the hostname you
//! configured `hostname@public-dns`. We recomend adding entries to your
//! `.ssh/config` file so that you can use shorthand to run commands like `ssh`,
//! and `scp` quickly once you are properly configured. Note that `.pem` private
//! key will need to be specified on the command line to authenticate yourself
//! when logging in.
//!
//! Here's a sample configuration:
//!
//! ```console
//! $ cat ~/.ssh/config
//! Host dev
//!   HostName ec2-xx-xx-xx-xx.us-west-2.compute.amazonaws.com
//!   Port 22
//!   User ubuntu
//!   IdentityFile <path to .pem file>
//! $ ssh dev
//! ```
//!
//! Setting up your VM is the same as setting up any other linux enviornment.
//! You will want to get your lab1 repository first. This will require you to
//! install git
//!
//! ```console
//! sudo apt install git
//! ```
//!
//! Run `git clone` as you would anywhere else to clone the repository.
//!
//! Working with code on remote machines can be really tricky if you've never
//! done it before. There are a variety of ways to do this. One nice way is to
//! edit files using VScode's remote editing features. If you configure your
//! `.ssh/config` correctly you should be able to ssh to a remote machine in the
//! editor and make changes to remote files as if they were local. When
//! deploying software to run on many virtual machines take care to ensure that
//! each is running the same version of your code.
//!
//! ## The Key-value Pair Service Interface
//!
//! The goal of Lab 1 is to wrap a key-value pair interface with RPC. You don't
//! need to implement the key-value pair storage by yourself, but you need to
//! use it extensively in later labs, so it will be good for you to understand
//! the service semantics here.
//!
//! The data structure and interfaces for the key-value pair service are defined
//! in the [tribbler::storage] module (in the [tribbler] crate). The main
//! interface is [tribbler::storage::Storage], which consists of three logical
//! parts.
//!
//! First is the key-string pair part, which is its own interface.
//!
//! ```rust
//! #[async_trait] // important annotation!
//! /// Key-value pair interfaces
//! /// Default value for all keys is empty string
//! pub trait KeyString {
//!     /// Gets a value. If no value set, return [None]
//!     async fn get(&self, key: &str) -> TribResult<Option<String>>;
//!
//!     /// Set kv.Key to kv.Value. return true when no error.
//!     async fn set(&self, kv: &KeyValue) -> TribResult<bool>;
//!
//!     /// List all the keys of non-empty pairs where the key matches
//!     /// the given pattern.
//!     async fn keys(&self, p: &Pattern) -> TribResult<List>;
//! }
//! ```
//!
//! [Pattern](tribbler::storage::Pattern) is a (prefix, suffix) tuple. It has a
//! `match` function that returns true when the string matches has the prefix
//! and suffix of the pattern.
//!
//! The second part is the key-list pair interface that handles list-valued
//! key-value pairs.
//!
//! ```rust
//! #[async_trait]
//! /// Key-list interfaces
//! pub trait KeyList {
//!     /// Get the list. Empty if not set.
//!     async fn list_get(&self, key: &str) -> TribResult<List>;
//!
//!     /// Append a string to the list. return true when no error.
//!     async fn list_append(&self, kv: &KeyValue) -> TribResult<bool>;
//!
//!     /// Removes all elements that are equal to `kv.value` in list `kv.key`
//!     /// returns the number of elements removed.
//!     async fn list_remove(&self, kv: &KeyValue) -> TribResult<u32>;
//!
//!     /// List all the keys of non-empty lists, where the key matches
//!     /// the given pattern.
//!     async fn list_keys(&self, p: &Pattern) -> TribResult<List>;
//! }
//! ```
//!
//! The `Storage` interface glues these two interfaces together, and also
//! includes an auto-incrementing clock feature.
//!
//! ```rust
//! #[async_trait]
//! pub trait Storage: KeyString + KeyList {
//!     /// Returns an auto-incrementing clock. The returned value of each call will
//!     /// be unique, no smaller than `at_least`, and strictly larger than the
//!     /// value returned last time, unless it was [u64::MAX]
//!     async fn clock(&self, at_least: u64) -> TribResult<u64>;
//! }
//! ```
//!
//! Note that the function signatures of these methods are already RPC-friendly.
//! You should implement the RPC interface with the rust-based gRPC framework
//! [`tonic`](https://docs.rs/tonic/latest/tonic/).  By doing this, another
//! person's client that speaks the same protocol will be able to talk to your
//! server as well.
//!
//! All errors you see from this interface will be communication errors. You can
//! assume that each call (on the same key) is an atomic transaction; two
//! concurrent writes won't give the key a weird value that came from nowhere.
//! However, when an error occurs, the caller won't know if the transaction
//! committed or not, because the error might have occured before or after the
//! transaction executed on the server.
//!
//! ## Rust and Async
//!
//! Rust has this neat keyword concept called `async`. Function scopes can be
//! marked as async which allows you to use a special keyword called `.await`.
//! Using `.await` on a function which returns a future utilizes an executor
//! (from a crate like [tokio]) to drive the IO to completion. Async functions
//! are implicitly wrapped by a [std::future::Future]. Calling `.await` on the
//! future will retrieve its value.
//!
//! ```rust
//! async fn do_io() -> bool {
//!     println!("look ma! I'm doin' I/O!");
//!     true
//! }
//! #[tokio::main]
//! async fn main() {
//!     let fute = do_io(); // type: Future<Output = bool>;
//!     let result = fute.await; // type: bool
//!     println!("we got the future value: {}", result);
//! }
//! ```
//!
//! This might sound and look confusing, but you don't need to understand the
//! internals of how this happens. Just understand that if you see `async fn`,
//! you can call `.await` inside of it. There's [an entirely separate book for
//! async
//! Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html).
//! You can read this if you want to understand more.
//!
//! Lastly, there's one more note about `async` in Rust. Currently, the language
//! is still partially in its infancy, especially the async features. The
//! compiler doesn't support `async` in trait functions, however, a [special
//! crate](async_trait) supplying the `#[async_trait::async_trait]` annotation
//! allows this through some syntactic sugar and tricky macros. You'll need to
//! use it when implementing parts of the lab.
//!
//! ## Entry Functions
//!
//! These are the two entry functions you need to implement for this Lab.  This
//! is how other people's code (and your own code in later labs) will use your
//! code.
//!
//! ### Server-side
//!
//! ```
//! pub async fn serve_back(config: BackConfig) -> TribResult<()>
//! ```
//!
//! This function creates an instance of a back-end server based on
//! configuration [tribbler::config::BackConfig]
//!
//! - `addr` is the address the server should listen on, in the form of
//!   `<host>:<port>`
//! - `storage` is the storage device you will use for storing data. You should
//!   not store persistent data anywhere else.
//! - `ready` is a channel for notifying the other parts in the program that the
//!   server is ready to accept RPC calls from the network (indicated by the
//!   server sending the value `true`) or if the setup failed (indicated by
//!   sending `false`). `ready` might be `None`, which means the caller does not
//!   care about when the server is ready.
//! - `shutdown` is another type of channel for receiving a shutdown
//! notification. when a message is received on this channel, the server should
//! shut down. **Hint**: take a look at
//! [serve_with_shutdown](tonic::transport::server::Router)
//!
//! This function should block indefinitely unless there is errors or the server
//! is sent a shutdown signal. It is `async`, you should be able to call
//! `.await` on futures within it.
//!
//! Note that you don't need to (and should not) implement the key-value pair
//! storage service yourself.  You only need to wrap the given
//! [tribbler::storage::Storage] with RPC, so that a remote client can access it
//! via the network.
//!
//! ### Client-side
//!
//! ```rust
//! pub async fn new_client(addr: &str) -> TribResult<Box<dyn Storage>>
//! ```
//!
//! This function takes `addr` in the form of `<host>:<port>`, and connects to
//! this address for an http RPC server. It returns an implementation of
//! [tribbler::storage::Storage], which will provide the interface, and forward
//! all calls as RPCs to the server. You can assume that `addr` will always be a
//! valid TCP address.
//!
//! Note that when [new_client] is called, the server may not have started yet.
//! While it is okay to try to connect to the server at this time, you should
//! not report any error if your attempt fails. It might be best to wait to
//! establish the connection until you need it to perform your first RPC
//! function call.
//!
//! ## RPCs via gRPC, protobuf, and tonic
//!
//! Rust doesn't have any functionality built in to the standard libary to make
//! writing RPCs easier, so we're using a few packages to reduce the amount of
//! code that you are required to write to complete the lab. Here are some terms
//! you should know:
//!
//! - **gRPC** is a language-agnostic RPC framework
//! - **protobuf** (or protocol buffers) is an RPC message and service interface
//!   language (`.proto` files) and a wire serialization format.
//! - **tonic** is a gRPC server and client implementation which has protobuf
//! compilation support baked in.
//!
//! Tonic is a gRPC server and client implementation written and designed in
//! rust. gRPC implementations use a protobuf (`.proto`) file to define message
//! types and RPC service interfaces. You can find the proto files for the
//! tribbler service defined in `tribbler/proto/rpc.proto`. During the build
//! process, cargo will compile the RPC definitions into [tribbler::rpc] . You
//! shouldn't need to make changes to either of these files to complete the lab.
//! The types defined in [tribbler::rpc] are used to implement the
//! tonic-specific RPC methods.
//!
//! ## Testing
//!
//! Defining unit tests in rust is simple. Write a `mod test {}` tests within
//! the module you wish to test using and annotate it with `#[cfg(test)]`. To
//! run the tests, you can then simply run `cargo test`. Example
//!
//! ```rust
//! // your code here
//! // ...
//!
//! #[cfg(test)]
//! mod test {
//!   #[test]
//!   fn my_test() {
//!     // test code here
//!   }
//! }
//!
//! // In your terminal
//! // $ cargo test
//! ```
//!
//! A non-exhaustive suite of tests are already provided inside of the
//! [lab](crate) module.
//!
//! When you implement the logic behind Lab 1, you should pass these tests, and
//! you can be fairly confident that you'll get at least 30% of the credit for
//! Lab 1 (assuming you're not cheating somehow).
//!
//! However, the tests that come with the repository are fairly basic and
//! simple. Though you're not required to, you should consider writing more test
//! cases to make sure your implementation matches the specification.
//!
//! For more information on writing tests in Rust, please read the
//! [testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
//! documentation in the Rust book.
//!
//! ## Starting Hints
//!
//! While you are free to do the project in your own way as long as it fits the
//! specification, matches the interfaces, and passes the tests, here are some
//! suggested first steps.
//!
//! First, create a `client.rs` file under the [crate::lab1] module, declare a
//! new struct called `StorageClient`, and make it visible to your lab 1 module:
//!
//! ```rust
//! // ... in lab/src/lab1/client.rs
//! pub struct StorageClient;
//! // ... in lab/src/lab1/mod.rs
//! mod client;
//! ```
//!
//! Then, add method implementations to this new `StorageClient` type so that it
//! matches the [tribbler::storage::Storage] trait. For example, for the `get()`
//! function:
//!
//! ```rust
//! impl KeyString for StorageClient {
//!     fn get(&self, key: &str) -> TribResult<Option<String>> {
//!         todo!()
//!     }
//! }
//! ```
//!
//! After you've added all of the functions with [todo!]s, you should have
//! client type that satisfies [tribbler::storage::Storage], we can return this
//! type in our entry function [new_client]. Remove the `todo!()` line in
//! [new_client], and replace it by returning a new `client` object. The
//! function returns a `TribResult<Box<dyn Storage>>` so we need to wrap our
//! type with `Ok(Box::new())` to satisfy the type constraints. Now the
//! [new_client] function should look something like this:
//!
//! ```rust
//! use crate::lab1::client::StorageClient;
//! pub async fn new_client(addr: &str) -> TribResult<Box<dyn Storage>> {
//!     Ok(Box::new(StorageClient { }))
//! }
//! ```
//!
//! Now all you need to do for the client half is to fill in the code skeleton
//! with the correct RPC logic, removing the [todo!]s along the way. You
//! probably will need to add fields to your struct. E.g. the `addr` parameter
//! is something you should probably store. Note that in rust [str] and [String]
//! are distinct types. [Read more
//! here](https://doc.rust-lang.org/book/ch08-02-strings.html). To convert from
//! `&str` to [String], call the `to_string()` method.
//!
//! ```rust
//! pub struct StorageClient {
//!     pub addr: String,
//! }
//!
//! // now when creating a new client:
//! // let _ = StorageClient { addr: addr.to_string() };
//! ```
//!
//! To implement the RPCs, we need to
//!
//! ```rust
//! use tribbler::rpc;
//! ```
//!
//! and then implement the methods on the
//! [tribbler::rpc::trib_storage_client::TribStorageClient] RPC client service
//! stub.
//!
//! The examples in the [tonic] documentation show how to write the basic RPC
//! client logic. However, most examples assume that we use an async runtime.
//! The interface given does not use the `async` keyword, so you need to make
//! sure these calls are blocking. You can use the [tokio] crate which is
//! responsible for managing async runtimes that handle executing the `async`
//! functions.
//!
//! Following their example, you could create a `get()` method that looks
//! something like this (though you shouldn't be connecting a new `client` on
//! every RPC):
//!
//! ```rust
//! use tribbler::rpc::trib_storage_client::TribStorageClient;
//! #[async_trait] // VERY IMPORTANT !!
//! impl KeyString for ClientStorage {
//!     async fn get(&self, key: &str) -> TribResult<Option<String>> {
//!         let mut client = TribStorageClient::connect(self.addr.clone()).await?;
//!         let r = client.get(Key {
//!             key: key.to_string(),
//!         }).await?;
//!         match r.into_inner().value {
//!             value => Ok(Some(value)),
//!         }
//!      }
//!   ...
//! }
//! ```
//!
//! However, if you do it this way, you will open a new HTTP connection for
//! every RPC call. This approach is acceptable but obviously not the most
//! efficient way available to you.  We leave it to you to figure out how to
//! maintain a persistent RPC connection, if it's something you want to tackle.
//!
//! We recommend reading the [tonic] documentation and examples from the
//! [project
//! repository](https://github.com/hyperium/tonic/tree/master/examples/src) to
//! see how to efficiently utilize the library.
//!
//! Once you've completed the client side, you also need to wrap the server side
//! in the [serve_back] function using the same [tribbler::rpc] module. This
//! should be similar to the examples in the `tonic` documentation. You do this
//! by creating a new struct for the RPC server, and implementing the the
//! `TribStorage` interface for the server side (from
//! [tribbler::rpc::trib_storage_server::TribStorage]). You should write a new
//! struct which implements this interface. Just remember that you need to send
//! a `true` over the `ready` [channel](std::sync::mpsc::channel) when the
//! service is ready (when `ready` is not `None`), and send a `false` when you
//! encounter any error on starting your service. The `shutdown` signal also
//! needs to be handled.
//!
//! When all of these changes are done, you should pass the test cases written
//! in the `lab1_test.rs` file. It performs some basic checks to see if an RPC
//! client and a server (that runs on the same host) will satisfy the
//! specification of a key-value pair service (as a local
//! [tribbler::storage::Storage] does without RPC).
//!
//!
//! ```console
//! # run the lab 1 tests
//! $ cargo test -p lab --tests -- --test-threads 1
//! ```
//!
//! ## Syntactic Sugar for RPC Server Implementations with `async_trait`
//!
//! It's very helpful to utilize the `#[async_trait]` mentioned earlier. While
//! implementing the storage server, You might encounter examples of
//! implementation that look like:
//!
//! ```rust
//! // StorageServer is a type you can define in your lab
//! impl KeyString for StorageServer {
//!     fn set<'life0, 'async_trait>(
//!         &'life0 self,
//!         request: tonic::Request<rpc::KeyValue>,
//!     ) -> core::pin::Pin<
//!         Box<
//!             dyn core::future::Future<Output = Result<tonic::Response<rpc::Bool>, tonic::Status>>
//!                 + core::marker::Send
//!                 + 'async_trait,
//!         >,
//!     >
//!     where
//!         'life0: 'async_trait,
//!         Self: 'async_trait,
//!     {
//!         todo!()
//!     }
//!     //... more implementations below
//! }
//! ```
//!
//! Gross! Reading that type signature can be quite difficult. All those
//! [core::pin::Pin] and [Box] types correspond to internal workings of `async`.
//!  Fortunately, `async_trait` can make it so that we don't need to worry about
//!  all that.
//!
//! Using the [async_trait::async_trait] above your `impl` results in methods
//! that can look like:
//!
//! ```
//! #[async_trait::async_trait]
//! impl KeyString for StorageServer {
//!     async fn set(&self, request: tonic::Request<rpc::KeyValue>,
//!     ) -> Result<tonic::Response<rpc::Bool>, tonic::Status> {
//!         todo!();
//!     }
//! }
//! ```
//!
//! Much better!
//!
//! ## Playing with your implementation
//!
//! To do some simple testing with your own implementation, you can use the
//! `kv-client` and `kv-server` command line utilities.
//!
//! First make sure your code compiles, then run the server.
//!
//! ```
//! $ cargo run --bin kv-server
//! ```
//!
//! You should see an address print out (e.g. `localhost:7799`). If desired, you
//! can override this setting with a command line flag.
//!
//! Now you can play with your server via the `kv-client` program. For example:
//!
//! ```
//! $ cargo run --bin kv-client
//! > set foo value
//! Ok(true)
//! > get foo
//! Ok(Some("value"))
//! > keys fo
//! Ok(List(["foo"]))
//! > list-get hello
//! Ok(List([]))
//! > list-get foo
//! Ok(List([]))
//! > list-append foo something
//! Ok(true)
//! > list-get foo
//! Ok(List(["something"]))
//! > clock
//! Ok(0)
//! > clock
//! Ok(1)
//! > clock
//! Ok(2)
//! > clock 200
//! Ok(200)
//! ```
//!
//! Both `kv-client` and `kv-server` have options which allow you to specify the
//! host address.
//!
//! ## Requirements
//!
//! - When the network and storage are errorless, RPC to your server should
//!   never return an error.
//! - When the network has an error (like the back-end server crashed, and thus
//!   the client cannot connect), your RPC client should return an error.  As
//!   soon as the server is back up and running, your RPC client should act as
//!   normal again (without needing to create a new client).
//! - When the server and the clients are running on the lab machines, your RPC
//!   should introduce less than 100 milliseconds of additional latency.
//!
//! ## Turning In Your Code
//!
//! Instructions for turning in the assignment are provided in the lab
//! repository.
//!
//! ## Happy Lab 1!
//!
mod lab;
pub use crate::lab1::lab::new_client;
pub use crate::lab1::lab::serve_back;
