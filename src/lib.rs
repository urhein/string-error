// Copyright 2017 Ulrich Rhein
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The `string-error` crate.
//!
//! This crate provides a simple way to use a string as an error
//! trait object, i.e. `Box<std::error::Error>`.
//!
//! If you need more sophisticated error handling, you should consider
//! [error-chain](https://crates.io/crates/error-chain), which also provides
//! functionality to create simple errors from Strings.

use std::fmt;
use std::error::Error;

/// Wraps `&'static str` and implements the `Error` trait for it.
#[derive(Debug)]
struct StaticStrError {
    error: &'static str
}

impl Error for StaticStrError {
    fn description(&self) -> &str {
        self.error
    }
}

impl fmt::Display for StaticStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error)
    }
}

/// Wraps an owned `String` and implements the `Error` trait for it.
#[derive(Debug)]
struct StringError {
    error: String
}

impl Error for StringError {
    fn description(&self) -> &str {
        &self.error
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.error)
    }
}

/// Creates an error trait object for a string constant (`&'static str`).
///
/// # Examples
///
/// ```
/// use string_error::*;
///
/// let x = static_err("Foo");
/// assert_eq!(x.description(), "Foo");
/// ```
pub fn static_err(e: &'static str) -> Box<Error> {
    Box::new(StaticStrError { error: e })
}

/// Creates an error trait object for a string (`&str`).
///
/// This copies the argument into an owned string. To avoid the copy, use
/// either `into_err` or `static_err`.
/// 
/// # Examples
///
/// ```
/// use string_error::*;
///
/// let x = new_err("Foo");
/// assert_eq!(x.description(), "Foo");
/// ```
pub fn new_err(e: &str) -> Box<Error> {
    Box::new(StringError { error: String::from(e) })
}

/// Creates an error trait object for an owned string (`String`).
///
/// This takes ownership of the `String` argument.
///
/// # Examples
///
/// ```
/// use string_error::*;
///
/// let x = into_err(String::from("Foo"));
/// assert_eq!(x.description(), "Foo");
/// ```
pub fn into_err(e: String) -> Box<Error> {
    Box::new(StringError { error: e })
}

#[cfg(test)]
mod tests {
    use super::*;
    static SOME_STRING : &'static str = "This is a String?!";

    #[test]
    fn test_static_err() {
        let x = static_err(SOME_STRING);
        assert_eq!(x.description(), SOME_STRING);
        assert!(x.cause().is_none());
    }

    #[test]
    fn test_new_err() {
        let x = new_err(SOME_STRING);
        assert_eq!(x.description(), SOME_STRING);
        assert!(x.cause().is_none());
    }

    #[test]
    fn test_into_err() {
        let x = into_err(String::from(SOME_STRING));
        assert_eq!(x.description(), SOME_STRING);
        assert!(x.cause().is_none());
    }
}
