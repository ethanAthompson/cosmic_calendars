//! This module contains shared code that's uniqified by a struct.
//!

#[derive(Clone, Debug)]
/// This struct represents the context of the navbar search
pub struct NavbarSearchString(pub String);

#[derive(Clone, Debug)]
/// This structure represents the context of a new user
pub struct NewUser(pub bool);
