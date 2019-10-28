#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(
    test(
        attr(allow(unused_variables),
        //deny(warnings)
    )
))]
#![feature(seek_convenience, const_fn)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //
    clippy::should_implement_trait,
    clippy::use_debug,
    clippy::decimal_literal_representation,
    clippy::option_unwrap_used,
    clippy::print_stdout,
    clippy::use_self,
    clippy::used_underscore_binding,
    clippy::unseparated_literal_suffix,
    clippy::type_repetition_in_bounds
)]
#![allow(clippy::must_use_candidate)] // this is annoying
//! This library provides a [`SubCursor`], that allows to only have access to
//! parts of a [`Read`]er or [`Write`]r.
//!
//! # Examples
//!
//! Creating a SubCursor from a type, that implements [`Read`] or [`Write`] and
//! [`Seek`].
//!
//! ```
//! use std::io;
//! use std::io::{Cursor, Read};
//!
//! use sub_cursor::SubCursor;
//!
//! fn main() -> io::Result<()> {
//!     let cursor = Cursor::new(b"Hello World!".to_vec());
//!     let mut sub_cursor = SubCursor::from(cursor).start(6);
//!
//!     let mut result = String::new();
//!     # let value =
//!     sub_cursor.read_to_string(&mut result)?;
//!     # assert_eq!(value, 6);
//!     assert_eq!(result, "World!".to_string());
//!
//!     Ok(())
//! }
//! ```
//!
//! You can spawn new [`SubCursor`] from other [`SubCursor`].
//! ```
//! use std::io::{self, Read};
//! use sub_cursor::SubCursor;
//!
//! fn main() -> io::Result<()> {
//!     let mut sub_cursor_1 = SubCursor::from(b"This is an example string.".to_vec());
//!     let mut sub_cursor_2 = sub_cursor_1.sub_cursor().start(5);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Planned Features
//! + `SubCursor[0..12]` syntax like with slices
//! + `no_std` support
//! + `AsyncRead` + `AsyncWrite` SubCursor
//! + travis integration
//! + `BufRead` support
//! + fix soundness around bounds and make integer conversions correct! (by that
//! I mean, that it's kind of undefined, what the maximum supported value is for
//! Seek, Write and Read and the functions might crash because of a broken
//! integer conversion...)
//!
//! [`Write`]: std::io::Write
//! [`Read`]: std::io::Read
//! [`Seek`]: std::io::Seek
pub mod prelude;
mod sub_cursor;

pub use crate::sub_cursor::*;

//
