//! # crate for regular expression engine
//!
//! ## example usage
//! ```
//! use myregex;
//! let expr = "a(bc)+|c(def)*"; 				// regular expression
//! let line = "cdefdefdef"; 					// string to match
//! myregex::do_matching(expr, line, true);		// match by dfs
//! myregex::print(expr);						// print AST of regular expression and sequence of instructions
//! ```
mod engine;
mod helper;

pub use engine::{do_matching, print};
