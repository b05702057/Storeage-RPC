//! This module contains implementation and functions for returning [std::error::Error] and [Result] type
//! objects from Tribbler related functions.
use std::{error::Error, fmt::Display};

/// basic error types that can occur when running the tribbler service.
#[derive(Debug, Clone)]
pub enum TribblerError {
    /// used when an operation is called for a particular user who does not
    /// exist
    UserDoesNotExist(String),
    /// when a user tries to sign up and the name is already taken
    UsernameTaken(String),
    /// when a username is invalid in any way
    InvalidUsername(String),
    /// generic error for anything that occurs with RPC communication
    RpcError(String),
    /// raised when too a user tries to follow more than
    /// [crate::trib::MAX_FOLLOWING]
    FollowingTooMany,
    /// raised when a user tries to follow a user they are already following
    AlreadyFollowing(String, String),
    /// raised when a user tries to unfollow a user they are not following
    NotFollowing(String, String),
    /// raised when a trib message exceeds [crate::trib::MAX_TRIB_LEN]
    TribTooLong,
    /// when someone tries to follow or check if a user is following themselves
    WhoWhom(String),
    /// when there are no more seq numbers to give out
    MaxedSeq,
    /// catch-all error for other issues
    Unknown(String),
}

impl Display for TribblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            TribblerError::UserDoesNotExist(x) => format!("user \"{}\" does not exist", x),
            TribblerError::UsernameTaken(x) => format!("username \"{}\" already taken", x),
            TribblerError::InvalidUsername(x) => format!("username \"{}\" is invalid", x),
            TribblerError::RpcError(x) => format!("rpc error: {}", x),
            TribblerError::FollowingTooMany => "following too many users".to_string(),
            TribblerError::AlreadyFollowing(who, whom) => {
                format!("{} already following {}", who, whom)
            }
            TribblerError::NotFollowing(who, whom) => format!("{} doesn't follow {}", who, whom),
            TribblerError::TribTooLong => "tribbler post exceed character limit".to_string(),
            TribblerError::WhoWhom(x) => format!("user {} can't follow themself", x),
            TribblerError::Unknown(x) => format!("unknown error: {}", x),
            x => format!("{:?}", x),
        };
        write!(f, "{}", x)
    }
}

impl std::error::Error for TribblerError {}

impl From<tonic::Status> for TribblerError {
    fn from(v: tonic::Status) -> Self {
        TribblerError::RpcError(format!("{:?}", v))
    }
}

impl From<tonic::transport::Error> for TribblerError {
    fn from(v: tonic::transport::Error) -> Self {
        TribblerError::RpcError(format!("{:?}", v))
    }
}

/// A [Result] type which either returns `T` or a [boxed error](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)
pub type TribResult<T> = Result<T, Box<(dyn Error + Send + Sync)>>;

impl From<Box<dyn Error>> for TribblerError {
    fn from(x: Box<dyn Error>) -> Self {
        TribblerError::Unknown(x.to_string())
    }
}
