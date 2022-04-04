//! Welcome to Lab 3! The goal of this lab is to take the bin storage that we
//! implemented in Lab 2 and make it fault-tolerant.
//!
//! Lab 3 can be submitted in teams of up to 3 people.
//!
//! ## Get Your Repo
//!
//! This is the the first assignment you will complete in groups. As a team you
//! will create a shared Github repository. Have one group member follow the
//! GitHub classroom link available through the Lab 3 assignment on Canvas. Once
//! one group member has made the group the others can select it. **Choose your
//! teammates carefully prior to accepting a Github group**, the choice is
//! binding. Once you have a repo select one of your group members to push their
//! code from lab2 to the freshly created lab3 starter repo for your team. See
//! this [Github
//! Documentation](https://docs.github.com/en/github/getting-started-with-github/managing-remote-repositories)
//! for how to perform the push. The commands to make the push are listed
//! explicitly in the lab3-starter Readme which you should have after cloning
//! the starter repo.
//!
//! Note that we don't provide great unit tests to test fault tolerance (as it's
//! hard to spawn and kill processes from within unit tests). Make sure you test
//! this sufficiently using a testing mechanism of your own design.
//!
//! ## System Scale and Failure Model
//!
//! There could be up to 300 backends. Backends may join and leave at will, but
//! you can assume that at any time there will be at least one backend online
//! (so that your system is functional). Your design is required to be
//! fault-tolerant where if there are at least three backends online at all
//! times, there will be no data loss. You can assume that each backend
//! join/leave event will have a time interval of at least 30 seconds in
//! between, and this time duration will be enough for you to migrate storage.
//!
//! There will be at least 1 and up to 10 keepers. Keepers may join and leave at
//! will, but at any time there will be at least 1 keeper online. (Thus, if
//! there is only one keeper, it will not go offline.) Also, you can assume that
//! each keeper join/leave event will have a time interval of at least 1 minute
//! in between. When a process 'leaves', assume that the process is killed--
//! everything in that process will be lost, and it will not have an opportunity
//! to clean up.
//!
//! When keepers join, they join with the same
//! [index](tribbler::config::KeeperConfig::this) as last time, although they've
//! lost any other state they may have saved. Each keeper will receive a new
//! [id](tribbler::config::KeeperConfig::id) in the
//! [KeeperConfig](tribbler::config::KeeperConfig).
//!
//! Initially, we will start at least one backend, and then at least one keeper.
//! At that point, the keeper should send `true` to the [ready
//! channel](tribbler::config::KeeperConfig::ready) and a frontend should be
//! able to issue BinStorage calls.
//!
//! ## Consistency Model
//!
//! To tolerate failures, you have to save the data of each key in multiple
//! places. To keep things achievable, we have to slightly relax the consistency
//! model, as follows.
//!
//! [`clock()`](tribbler::storage::Storage::clock) and the key-value calls
//! ([`set()`][s], [`get()`][g], and [`keys()`][k]) will keep the same semantics
//! as before.
//!
//!
//! When concurrent [`list_appends()`s][la] happen, calls to [`list_get()`][lg]
//! might result in values that are currently being added, and may appear in
//! arbitrary order. However, after all concurrent [`list_append()`s][la]
//! return, [`list_get()`][lg] should always return the list with a consistent
//! order.
//!
//! Here is an example of a valid call and return sequence:
//!
//! - Initially, the list "k" is empty.
//! - A invokes `list_append("k", "a")`
//! - B invokes `list_append("k", "b")`
//! - C calls `list_get("k")` and gets `["b"]`. Note that `"b"` appears first in
//!   the list here.
//! - D calls `list_get("k")` and gets `["a", "b"]`, note that although `"b"`
//!   appeared first last time, it appears at the second position in the list
//!   now.
//! - A's `list_append()` call returns
//! - B's `list_append()` call returns
//! - C calls `list_get("k")` again and gets `["a", "b"]`
//! - D calls `list_get("k")` again and gets `["a", "b"]`
//!
//! [`list_remove()`][lr] removes all matched values that are appended into the
//! list in the past, and returns number properly. When (and only when)
//! concurrent [`list_remove()`s][lr] on the same key and value is called, it is
//! okay to 'double count' elements being removed.
//!
//! [`list_keys()`][lk] keeps the same semantics.
//!
//! [s]: tribbler::storage::KeyString::set
//! [g]: tribbler::storage::KeyString::get
//! [k]: tribbler::storage::KeyString::keys
//! [la]: tribbler::storage::KeyList::list_append
//! [lg]: tribbler::storage::KeyList::list_get
//! [lr]: tribbler::storage::KeyList::list_remove
//! [lk]: tribbler::storage::KeyList::list_keys
//!
//! ## Entry Functions
//!
//! The entry functions will remain exactly the same as they are in Lab
//! 2. The only thing that will change is that there may be multiple keepers
//! listed in the [KeeperConfig](tribbler::config::KeeperConfig::addrs).
//!
//! ## Additional Assumptions
//!
//! - No network errors; when a TCP connection is lost (RPC client returning an
//!   [error status code](tonic::Status)), you can assume that the RPC server
//!   crashed.
//! - When a bin-client, backend, or keeper is killed, all data in that process
//!   will be lost; nothing will be carried over a respawn.
//! - It will take less than 20 seconds to read all data stored on a backend and
//!   write it to another backend.
//!
//! ## Requirements
//!
//! - Although you might change how data is stored in the backends, your
//!   implementation should pass all past test cases, which means your system
//!   should be functional with a single backend.
//! - If there are at least three backends online, there should never be any
//!   data loss. Note that the set of three backends might change over time, so
//!   long as there are at least three at any given moment.
//! - Assuming there are backends online, storage function calls always return
//!   without error, even when a node and/or a keeper just joined or left.
//!
//! ## Building Hints
//!
//! - You can use the logging techniques described in class to store everything
//!   (in lists on the backends, even for values).
//! - Let the keeper(s) keep track on the status of all the nodes, and do the
//!   data migration when a backend joins or leaves.
//! - Keepers should also keep track of the status of each other.
//!
//! For the ease of debugging, you can maintain some log messages (by using the
//! [log] crate, or by writing to a TCP socket or a log file). However, for the
//! convenience of grading, please turn them off by default when you turn in
//! your code.
//!
//! ## `Report.md`
//!
//! Similar to in Lab 2, please include a report file. See the [description in
//! Lab 2](crate::lab2) for more details.
//!
//! ## Turning In
//!
//! Each student must submit their own version of the group repository to
//! Gradescope by the deadline (+their individual late hours). Every member of
//! the team can submit the exact same repository commit. Individual submissions
//! are being used to allow individual teammates to make use of their late hours
//! if they feel the need.
//!
//! Happy Lab 3. :-)
