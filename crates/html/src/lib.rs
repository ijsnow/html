//! Typed HTML support for Rust.
//!
//! # Philosophy
//!
//! HTML is easy to get started with, but hard to get right. There are several
//! hundred element kinds, element attributes, and deeply nested hierachies - with
//! some relationships even being conditional on each other. Remembering all of this
//! is difficult and error-prone, but luckily we don't have to remember any of this
//! by using the type system! Rust's type system enables us to model the entire HTML
//! spec, allowing us to catch all errors ahead of time during compilation.
//!
//! This project comes in layers. The bottom-most layer is the HTML spec itself. We
//! download it, and parse it into definition files. We then take these definitions,
//! and use it to generate the `html-sys` crate. This crate is semantically correct,
//! and knows how to render itself to string representations. We then combine
//! `html-sys` with `web-sys` (wip) to create a higher-level HTML interface,
//! complete with support for events. This can be used to manipulate HTML both in
//! browser (wip) and non-browser contexts.
//!
//! # Examples
//!
//! We can create HTML structures one-by-one:
//! ```rust
//! # #![allow(unused)]
//! #![recursion_limit = "512"]
//!
//! use html::text_content::OrderedList;
//! let tree = OrderedList::builder()
//!     .list_item(|li| li.text("nori").class("cat"))
//!     .list_item(|li| li.text("chashu").class("cat"))
//!     .build();
//! let string = tree.to_string();
//! ```
//! But we can also use Rust's native control flow structures such as loops to
//! iterate over items and create HTML:
//! ```rust
//! # #![allow(unused)]
//! #![recursion_limit = "512"]
//!
//! use html::text_content::OrderedList;
//! let mut ol = OrderedList::builder();
//! for name in ["hello", "world"] {
//!     ol.list_item(|li| li.text(name));
//! }
//! let tree = ol.build();
//! ```
//!
//! We can also create elements separately and append them later:
//! ```rust
//! # #![allow(unused)]
//! #![recursion_limit = "512"]
//!
//! use html::text_content::{OrderedList, ListItem};
//! let mut ol = OrderedList::builder();
//! let li = ListItem::builder().text("hello").build();
//! ol.push(li);
//! let tree = ol.build();
//! ```

#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
// #![deny(missing_debug_implementations, nonstandard_style)]
#![warn(future_incompatible, rust_2018_idioms)]
#![warn(missing_docs)]

mod generated;
mod manual;

use std::borrow::Cow;
use std::collections::HashMap;

pub use manual::categories::*;

pub use manual::content;
pub use manual::edits;
pub use manual::embedded;
pub use manual::forms;
pub use manual::inline_text;
pub use manual::interactive;
pub use manual::media;
pub use manual::metadata;
pub use manual::root;
pub use manual::scripting;
pub use manual::tables;
pub use manual::text_content;
pub use manual::web_components;

/// Render an HTML element to a string.
///
/// This API is similar to `Display`, but it takes a `depth` argument which
/// allows rendered items to be indented.
///
/// Users of this crate are expected to keep using the `Display` interface as
/// normal. This trait only exists for internal bookkeeping.
pub trait Render {
    /// Render an element with a given `depth` argument.
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result;
}

impl Render for Cow<'static, str> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        write!(f, "{:level$}", "", level = depth * 4)?;
        std::fmt::Display::fmt(self, f)
    }
}

impl<T> Render for &T
where
    T: Render + ?Sized,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        Render::render(&**self, f, depth)
    }
}
impl<T> Render for &mut T
where
    T: Render + ?Sized,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        Render::render(&**self, f, depth)
    }
}

/// An HTML Element
pub trait HtmlElement {
    /// Get the tag name for an element.
    fn tag_name(&self) -> &'static str;

    /// Get the attributes for an element.
    fn attributes(&self) -> HashMap<Cow<'static, str>, Cow<'static, str>>;

    /// Get the dataset attributes for an element.
    fn data(&self) -> HashMap<Cow<'static, str>, Cow<'static, str>>;

    /// Get the children for an element.
    fn children<'a>(&'a self) -> Vec<Node<'a>>;
}

/// A generic node element to describe children.
pub enum Node<'a> {
    /// An HtmlElement.
    Element(&'a dyn HtmlElement),
    /// A text node.
    Text(Cow<'static, str>),
}

impl<'a> From<&'a std::borrow::Cow<'static, str>> for Node<'a> {
    fn from(text: &'a std::borrow::Cow<'static, str>) -> Node<'a> {
        Node::Text(text.to_owned())
    }
}

impl From<std::borrow::Cow<'static, str>> for Node<'_> {
    fn from(text: std::borrow::Cow<'static, str>) -> Node<'_> {
        Node::Text(text)
    }
}
