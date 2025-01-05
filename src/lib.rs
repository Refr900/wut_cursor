//! # Wut Cursor
//! 
//! A lightweight iterator over a sequence of characters.
//! 
//! ## Example
//! 
//! ```rust
//! use wut_cursor::Cursor;
//! 
//! fn main() {
//!     let source = r#"
//!     fn main() {
//!         println!("Hello world!");
//!     }
//!     "#
//!     .trim_start();
//! 
//!     let mut cursor = Cursor::new(source);
//!     let mut index = 0;
//!     let mut count = 1;
//!     loop {
//!         let token = cursor.next();
//!         let end = index + token.len as usize;
//!         let lexeme = &source[index..end];
//!         println!("{:>3}: {:?}", count, token.kind);
//!         println!("   |  lexeme: {:?}", lexeme);
//!         if token.is_eof() {
//!             break;
//!         }
//!         index = end;
//!         count += 1;
//!     }
//! }
//! ```
//! 
//! ## Subtleties
//! 
//! By default, the cursor does not parse strings or characters, but has a default implementation that supports escaped symbols.
//! 
//! ```rust
//! use wut_cursor::{Cursor, Kind};
//! 
//! fn main() {
//!     let source = r#"
//!     fn main() {
//!         let c = '\0';
//!         print!("Hello world!\n");
//!     }
//!     "#
//!     .trim_start();
//! 
//!     let mut cursor = Cursor::new(source);
//!     let mut index = 0;
//!     let mut count = 1;
//!     loop {
//!         let token = cursor.next();
//!         let len = match token.kind {
//!             Kind!['"'] => {
//!                 // ready-made string parsing
//!                 let str = cursor.parse_str_continue();
//!                 str.len
//!             }
//!             Kind!['\''] => {
//!                 // ready-made character parsing
//!                 let char = cursor.parse_char_continue();
//!                 char.len
//!             }
//!             _ => token.len,
//!         };
//! 
//!         let end = index + len as usize;
//!         let lexeme = &source[index..end];
//!         println!("{:>3}: {:?}", count, token.kind);
//!         println!("   |  lexeme: {:?}", lexeme);
//! 
//!         if token.is_eof() {
//!             break;
//!         }
//!         index = end;
//!         count += 1;
//!     }
//! }
//! ```
//! 
//! And yes, there is a macro for the token kind.
#![forbid(unsafe_code)]

mod cursor;
mod token;

pub use cursor::*;
pub use token::*;

mod macros;

mod test;
