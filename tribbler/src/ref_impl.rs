//! This module contains a reference implementation for the single-node tribbler
//! service.
#![allow(dead_code)]
use std::{
    cmp::{min, Ordering},
    collections::{HashMap, HashSet},
    sync::{
        atomic::{self, AtomicU64},
        Arc, RwLock,
    },
};

use crate::{
    err::{TribResult, TribblerError},
    trib::{is_valid_username, Server, Trib, MAX_TRIB_FETCH, MAX_TRIB_LEN, MIN_LIST_USER},
};

/// The [User] type holds the data on tribs the user has posted along with
/// related follower information.
#[derive(Debug)]
struct User {
    following: HashSet<String>,
    followers: HashSet<String>,
    seq_tribs: Vec<SeqTrib>,
    tribs: Vec<Arc<Trib>>,
}

/// A [Trib] type with an additional sequence number
#[derive(Debug, Clone)]
struct SeqTrib {
    seq: u64,
    trib: Arc<Trib>,
}

impl Ord for SeqTrib {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seq.cmp(&other.seq)
    }
}

impl Eq for SeqTrib {}

impl PartialOrd for SeqTrib {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.seq.partial_cmp(&other.seq)
    }
}

impl PartialEq for SeqTrib {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl User {
    /// creates a new user reference
    fn new() -> User {
        User {
            following: HashSet::new(),
            followers: HashSet::new(),
            seq_tribs: vec![],
            tribs: vec![],
        }
    }

    /// Checks whether this user is following `whom`
    fn is_following(&self, whom: &str) -> bool {
        self.following.contains(whom)
    }

    /// updates [User] to follow `whom`
    fn follow(&mut self, whom: &str) {
        self.following.insert(whom.to_string());
    }

    /// updates [User] to unfollow `whom`
    fn unfollow(&mut self, whom: &str) {
        self.following.remove(whom);
    }

    /// updates [User] to add to the follower list
    fn add_follower(&mut self, who: &str) {
        self.followers.insert(who.to_string());
    }

    /// updates [User] to remove from the follower list
    fn remove_follower(&mut self, who: &str) {
        self.followers.remove(who);
    }

    /// lists the [User]s that this user follows
    fn list_following(&self) -> Vec<String> {
        self.following.iter().map(String::clone).collect()
    }

    /// instructs this [User] to post a new [Trib] with the given parameters
    /// returns a reference to the posted [Trib]
    fn post(&mut self, who: &str, msg: &str, seq: u64, time: u64) -> Arc<Trib> {
        // make the new trib
        let trib = Arc::new(Trib {
            user: who.to_string(),
            message: msg.to_string(),
            time,
            clock: seq,
        });
        // append sequential number
        let seq_trib = SeqTrib {
            seq,
            trib: trib.clone(),
        };

        // add to my own tribs
        self.tribs.push(trib.clone());
        self.seq_tribs.push(seq_trib);
        trib
    }

    /// Gets the list of [Trib]s posted by this [User]
    fn list_tribs(&self) -> &[Arc<Trib>] {
        let ntrib = self.tribs.len();
        let start = match ntrib.cmp(&MAX_TRIB_FETCH) {
            Ordering::Greater => ntrib - MAX_TRIB_FETCH,
            _ => 0,
        };
        &self.tribs[start..]
    }
}

/// The [RefServer] is a reference implementation for the [crate::trib::Server]
///
/// This struct should be able to be used across threads when wrapped with an
/// [Arc] type. Internal method implementation utilize locks which make mutations
/// thread-safe.
///
/// This reference implementation also eschews proper error handling when
/// one of the internal mutexes fails to lock the data. This methods on this
/// struct may cause a panic if a locking operation fails.
///
/// ```rust
/// use std::sync::Arc;
/// use std::thread;
/// use tribbler::ref_impl::RefServer;
/// use tribbler::trib::Server;
///
/// let server = Arc::new(RefServer::default());
/// let s = server.clone();
/// let handle = thread::spawn(move ||{
///     s.sign_up("user2");
/// });
/// server.sign_up("user1");
/// let _ = handle.join();
/// assert_eq!(server.list_users().unwrap().len(), 2);
///
/// ```
pub struct RefServer {
    users: Arc<RwLock<HashMap<String, User>>>,
    homes: Arc<RwLock<HashMap<String, Vec<Arc<Trib>>>>>,
    seq: AtomicU64,
}

impl RefServer {
    /// Creates a [RefServer] with no data
    pub fn new() -> RefServer {
        RefServer {
            users: Arc::new(RwLock::new(HashMap::new())),
            homes: Arc::new(RwLock::new(HashMap::new())),
            seq: AtomicU64::new(0),
        }
    }

    /// rebuilds the users' homepage based on the current set of [SeqTrib]s and
    /// other users' tribs
    fn rebuild_home(&self, who: &User, users: &HashMap<String, User>) -> Vec<Arc<Trib>> {
        let mut home: Vec<SeqTrib> = vec![];
        home.append(&mut who.seq_tribs.clone());
        for user in who.following.iter() {
            match users.get(user) {
                Some(v) => {
                    home.append(&mut v.seq_tribs.clone());
                }
                None => continue,
            };
        }
        home.sort();
        home.iter()
            .map(|x| x.trib.clone())
            .collect::<Vec<Arc<Trib>>>()
    }
}

impl Default for RefServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Server for RefServer {
    fn sign_up(&self, user: &str) -> TribResult<()> {
        let mut users = self.users.write().unwrap();
        if !is_valid_username(user) {
            return Err(Box::new(TribblerError::InvalidUsername(user.to_string())));
        }
        match users.contains_key(user) {
            true => Err(Box::new(TribblerError::UsernameTaken(user.to_string()))),
            false => {
                users.insert(user.to_string(), User::new());
                let mut homes = self.homes.write().unwrap();
                homes.insert(user.to_string(), vec![]);
                Ok(())
            }
        }
    }

    fn list_users(&self) -> TribResult<Vec<String>> {
        let users = self.users.read().unwrap();
        let mut k: Vec<&String> = users.keys().collect();
        k.sort();
        let sorted = k[..min(MIN_LIST_USER, k.len())].to_vec();
        let res: Vec<String> = sorted
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        Ok(res)
    }

    fn post(&self, who: &str, post: &str, clock: u64) -> TribResult<()> {
        if post.len() > MAX_TRIB_LEN {
            return Err(Box::new(TribblerError::TribTooLong));
        }
        let mut users = self.users.write().unwrap();
        match users.get_mut(who) {
            Some(user) => {
                if self.seq.load(atomic::Ordering::SeqCst) == u64::MAX {
                    return Err(Box::new(TribblerError::MaxedSeq));
                }
                let _ = self.seq.fetch_update(
                    atomic::Ordering::SeqCst,
                    atomic::Ordering::SeqCst,
                    |v| {
                        if v < clock {
                            Some(clock)
                        } else {
                            None
                        }
                    },
                );

                let trib = user.post(
                    who,
                    post,
                    self.seq.fetch_add(1, atomic::Ordering::SeqCst),
                    clock,
                );
                // add it to the timeline of my followers
                let mut homes = self.homes.write().unwrap();
                for follower in user.followers.iter() {
                    homes
                        .entry(follower.to_string())
                        .and_modify(|e| e.push(trib.clone()));
                }
                // add it to my own timeline
                homes
                    .entry(who.to_string())
                    .and_modify(|e| e.push(trib.clone()));
                Ok(())
            }
            None => Err(Box::new(TribblerError::UserDoesNotExist(who.to_string()))),
        }
    }

    fn tribs(&self, user: &str) -> TribResult<Vec<Arc<Trib>>> {
        let users = self.users.read().unwrap();
        match users.get(user) {
            Some(user) => Ok(user.list_tribs().to_vec()),
            None => Err(Box::new(TribblerError::UserDoesNotExist(user.to_string()))),
        }
    }

    fn follow(&self, who: &str, whom: &str) -> TribResult<()> {
        if who == whom {
            return Err(Box::new(TribblerError::WhoWhom(who.to_string())));
        }
        let mut users = self.users.write().unwrap();
        if !users.contains_key(whom) {
            return Err(Box::new(TribblerError::UserDoesNotExist(who.to_string())));
        }
        match users.get_mut(who) {
            Some(u) => {
                if u.is_following(whom) {
                    return Err(Box::new(TribblerError::AlreadyFollowing(
                        who.to_string(),
                        whom.to_string(),
                    )));
                }
                u.follow(whom);
            }
            None => return Err(Box::new(TribblerError::UserDoesNotExist(who.to_string()))),
        };
        let _ = users
            .entry(whom.to_string())
            .and_modify(|e| e.add_follower(who));
        // rebuild home
        match users.get(who) {
            Some(user) => {
                let mut homes = self.homes.write().unwrap();
                homes.insert(who.to_string(), self.rebuild_home(user, &users));
                Ok(())
            }
            None => Err(Box::new(TribblerError::UserDoesNotExist(who.to_string()))),
        }
    }

    fn unfollow(&self, who: &str, whom: &str) -> TribResult<()> {
        if who == whom {
            return Err(Box::new(TribblerError::WhoWhom(who.to_string())));
        }
        let mut users = self.users.write().unwrap();
        if !users.contains_key(whom) {
            return Err(Box::new(TribblerError::UserDoesNotExist(whom.to_string())));
        }
        match users.get_mut(who) {
            Some(u) => {
                if !u.is_following(whom) {
                    return Err(Box::new(TribblerError::NotFollowing(
                        who.to_string(),
                        whom.to_string(),
                    )));
                }
                u.unfollow(whom);
            }
            None => return Err(Box::new(TribblerError::UserDoesNotExist(whom.to_string()))),
        };
        let _ = users
            .entry(whom.to_string())
            .and_modify(|e| e.remove_follower(who));
        // rebuild home
        match users.get(who) {
            Some(user) => {
                let mut homes = self.homes.write().unwrap();
                homes.insert(who.to_string(), self.rebuild_home(user, &users));
                Ok(())
            }
            None => Err(Box::new(TribblerError::UserDoesNotExist(who.to_string()))),
        }
    }

    fn is_following(&self, who: &str, whom: &str) -> TribResult<bool> {
        if who == whom {
            return Err(Box::new(TribblerError::WhoWhom(who.to_string())));
        }
        let users = self.users.read().unwrap();
        if !users.contains_key(whom) {
            return Err(Box::new(TribblerError::UserDoesNotExist(whom.to_string())));
        }
        match users.get(who) {
            Some(user) => Ok(user.is_following(whom)),
            None => Err(Box::new(TribblerError::UserDoesNotExist(who.to_string()))),
        }
    }

    fn following(&self, who: &str) -> TribResult<Vec<String>> {
        let users = self.users.read().unwrap();
        match users.get(who) {
            Some(user) => Ok(user.list_following()),
            None => Err(Box::new(TribblerError::UserDoesNotExist(who.to_string()))),
        }
    }

    fn home(&self, user: &str) -> TribResult<Vec<Arc<Trib>>> {
        let homes = self.homes.read().unwrap();
        match homes.get(user) {
            Some(home) => {
                let ntrib = home.len();
                let start = match ntrib.cmp(&MAX_TRIB_FETCH) {
                    Ordering::Greater => ntrib - MAX_TRIB_FETCH,
                    _ => 0,
                };
                // let hm = &home[start..];
                Ok(home[start..].to_vec())
            }
            None => Err(Box::new(TribblerError::UserDoesNotExist(user.to_string()))),
        }
    }
}
