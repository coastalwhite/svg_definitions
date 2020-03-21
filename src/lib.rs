//! Hello fellow Rustacians! Here is a crate with SVG Definitions.
//! This was mostly created to serve as a backend crate for [wasm_svg_graphics](https://crates.io/crates/wasm_svg_graphics),
//! but feel free to use it!
//!
//! I am open to pull requests so please contribute!
//!
//!
//! # Example
//! ## Creating a group with a triangle
//! ```
//! use svg_definitions::prelude::*;
//!
//! let triangle = SVGElem::new(Tag::Path)
//!     .set(Attr::StrokeWidth, 1)
//!     .set(Attr::Stroke, "#000")
//!     .set(Attr::Fill, "transparent")
//!     .set(Attr::D, PathData::new()
//!         .move_to((0.0, 0.0))
//!         .line_to((10.0, 0.0))
//!         .line_to((0.0, 10.0))
//!         .line_to((0.0, 0.0))
//!         .close_path()
//!     );
//!
//! let group = SVGElem::new(Tag::G)
//!     .append(triangle);
//! ```
//!
//! ## Getting a svg from a file
//! *The feature "parsing" needs to be enabled for this*
//! ```
//! use svg_definitions::prelude::*;
//!
//! let shape = SVGParseFile("/path/to/file.svg");
//!
//! // ...
//! ```
//!
//! ## Getting a svg from text
//! *The feature "parsing" needs to be enabled for this*
//! ```
//! use svg_definitions::prelude::*;
//!
//! let rect = SVGParseText("<rect width=\"50px\" height=\"50\" fill=\"black\" />");
//!
//! // ...
//! ```

pub mod prelude;

pub mod attributes;
pub mod path;
pub mod tag_name;

#[cfg(feature = "parsing")]
pub mod parser;

pub type Point2D = (f32, f32);

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use attributes::Attribute;
use tag_name::TagName;

type Attributes = HashMap<Attribute, String>;
type Children = Vec<Element>;

/// Element provides a way to simulate DOM SVG elements
#[derive(Debug)]
pub struct Element {
    tag_name: TagName,
    attributes: Attributes,
    children: Children,
    inner: Option<String>,
}

// Implementation of Element
impl Element {
    /// Creates a new Element with a certain tag_name
    pub fn new(tag_name: TagName) -> Element {
        Element {
            tag_name,
            attributes: HashMap::new(),
            children: Vec::new(),
            inner: None,
        }
    }

    /// Appends an element to the children of the self element
    /// and consumes both whilst returning the product
    pub fn append(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }

    fn is_allowed_inner(text: &str) -> bool {
        let regular_expression =
            regex::Regex::new(r#"[a-zA-Z0-9' \\-_/.!?:;(){}[\\]`~&,"]+"#).unwrap();
        regular_expression.is_match(text)
    }

    /// Sets the inner text to a plain string
    /// Allowed characters are *a-zA-Z0-9'" -_/\.!?:;(){}[]`~&,*
    pub fn set_inner(mut self, text: &str) -> Self {
        if !Element::is_allowed_inner(text) {
            return self;
        }
        self.inner = Some(String::from(String::from(text).trim()));
        self
    }

    /// Sets an attribute of the self element to a certain value
    pub fn set<T>(mut self, attribute: Attribute, value: T) -> Self
    where
        T: ToString,
    {
        self.attributes.insert(attribute, value.to_string());
        self
    }

    /// Gets an immutable reference to the tag_name of this Element
    pub fn get_tag_name(&self) -> &TagName {
        &self.tag_name
    }

    /// Gets an immutable reference to the attributes of this Element
    pub fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Gets an immutable reference to the children of this Element
    pub fn get_children(&self) -> &Children {
        &self.children
    }

    /// Gets a clone of the inner text
    pub fn get_inner(&self) -> &Option<String> {
        &self.inner
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        let mut elem = Element::new(self.tag_name);
        for (key, value) in self.attributes.iter() {
            elem.attributes.insert(key.clone(), value.clone());
        }
        for child in self.children.iter() {
            elem = elem.append(child.clone());
        }
        elem
    }
}

impl Hash for Element {
    fn hash<T: Hasher>(&self, state: &mut T) {
        self.tag_name.hash(state);
        self.attributes.iter().for_each(|(key, value)| {
            key.hash(state);
            value.hash(state);
        });
        self.children.iter().for_each(|child| child.hash(state));
    }
}

impl Into<Element> for TagName {
    fn into(self) -> Element {
        Element::new(self)
    }
}
