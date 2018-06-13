//! # Model
//! Contains the data models used throughout the library and exposed through the public API.

extern crate chrono;
use self::chrono::{DateTime, Utc};
type Date = DateTime<Utc>;

/// Stores metadata about a specific user.
pub struct User {
    pub id: i32,
    pub handle: String,
    pub access: i32,
    pub added: Date,
}

/// Stores metadata about a specific challenge.
pub struct Challenge {
    pub id: i32,
    pub cid: i32,
    pub eid: i32,
    pub points: i32,
    pub name: String,
    pub url: String,
    pub notes: String,
    pub flag: String,
    pub added: Date,
}

/// A `Breadcrumb` associates a timestamp to a value.
pub struct Breadcrumb<T> {
    value: T,
    date: Date,
}

/// Historical trail of events
pub enum History<'a> {
    Added(Breadcrumb<&'a Challenge>),
    Updated(Breadcrumb<&'a Challenge>),
}

/// Stores metadata about a specific CTF event.
pub struct Event {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub start: Date,
    pub end: Date,
}

/// A submission to a challenge, for statistical purposes.
struct Submission {
    id: i32,
    cid: i32,
    flag: String,
    date: Date,
    valid: bool,
}

