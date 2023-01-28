//! # Crate for a Regex Engine
//!
//! ## Usage
//!
//! ```
//! use regex_zero;
//! let expr = "a(bc)+|c(def)*"; // a regex
//! let line = "cdefdefdef"; // the string to match the regex
//! regex_zero::do_matching(expr, line, true); // do matching by depth-first search
//! regex_zero::print(expr); // show the AST of the regex and the order seq
//! ```
mod engine;
mod helper;

pub use engine::{do_matching, do_matching_with_cache, print};
