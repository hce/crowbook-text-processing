// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Some utilities functions for processing texts.
//!
//! These functions were originally written for
//! [Crowbook](https://github.com/lise-henry/crowbook), but have
//! been published on a separate crate and under a less restrictive
//! license (MPL instead of LGPL) so they can be used elsewhere.
//!
//! # Example
//!
//! ```
//! use crowbook_text_processing::{FrenchFormatter, clean};
//! use crowbook_text_processing::{
//!     escape_html,
//!     escape_tex,
//! };
//!
//! let s = " Some  string with  too much   whitespaces & around 1% \
//!          characters that might cause trouble to HTML or LaTeX.";
//! // Remove unnecessary whitespaces (but doesn't trim at is can have meaning)
//! let new_s = clean::whitespaces(s);
//! // Escape forHTML
//! println!("for HTML: {}", escape_html(new_s.clone()));
//! // Escape for LaTeX
//! println!("for LaTeX: {}", escape_tex(new_s));
//!
//! // Replace quotes with typographic quotation marks
//! let s = r#"Some "quoted string" and 'another one'."#;
//! let new_s = clean::quotes(s);
//! assert_eq!(&new_s, "Some “quoted string” and ‘another one’.");
//!
//! // Replace three consecutive dots with ellipsis character
//! let s = clean::ellipsis("Foo...");
//! assert_eq!(&s, "Foo…");
//!
//! // Format whitespaces according to french typographic rules, using
//! // the appropriate non-breaking spaces where needed
//! let s = " Une chaîne en français ! On voudrait un résultat \
//!          « typographiquement correct ».";
//! let french = FrenchFormatter::new();
//! println!("for text: {}", french.format(s));
//! println!("for LaTeX: {}", escape_tex(french.format_tex(s)));
//! ```
//! # Requirements
//!
//! * `rustc >= 1.6.0`
//!
//! # Semantic versioning
//!
//! While not yet at version `1.0`, this crates tries to follows semantic
//! versioning in the following way:
//!
//! * an increase of `x` in `0.x.y` means breaking changes.
//! * an increase of `y` in `0.x.y` means non-breaking changes.
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod escape;
pub mod clean;

mod french;
mod common;

pub use escape::{escape_html, escape_tex};
pub use french::FrenchFormatter;
