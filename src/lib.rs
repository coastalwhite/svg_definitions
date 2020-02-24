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
//! let triangle = SVGElem::new(Tag::SVGPath)
//!     .set(Attr::StrokeWidth, 1.into())
//!     .set(Attr::StrokeColor, RGB::new(0,0,0).into())
//!     .set(Attr::FillColor, RGBT::Transparent.into())
//!     .set(Attr::PathDefinition, PathString::new()
//!         .move_to((0.0, 0.0))
//!         .line_to((10.0, 0.0))
//!         .line_to((0.0, 10.0))
//!         .line_to((0.0, 0.0))
//!         .close_path()
//!         .into()
//!     );
//!
//! let group = SVGElem::new(Tag::Group)
//!     .append(triangle);
//! ```

pub mod attributes;
pub mod color;
pub mod path;
pub mod prelude;

use std::clone::Clone;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use attributes::{Attribute, AttributeValue};

type Attributes = HashMap<Attribute, AttributeValue>;
type Children = Vec<Element>;

/// TagName provides tags for SVG creation
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TagName {
    SVG,
    Defs,
    SVGPath,
    Circle,
    Rectangle,
    Group,
    Use,
}

/// Element provides a way to simulate DOM SVG elements
#[derive(Debug)]
pub struct Element {
    tag_name: TagName,
    attributes: Attributes,
    children: Children,
}

// Implementation of Tagname
impl ToString for TagName {
    fn to_string(&self) -> String {
        use TagName::*;

        String::from(match self {
            SVG => "svg",
            Defs => "defs",
            SVGPath => "path",
            Circle => "circle",
            Rectangle => "rect",
            Group => "g",
            Use => "use",
        })
    }
}

// Implementation of Element
impl Element {
    /// Creates a new Element with a certain tag_name
    pub fn new(tag_name: TagName) -> Element {
        Element {
            tag_name,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Appends an element to the children of the self element
    /// and consumes both whilst returning the product
    pub fn append(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    /// Sets an attribute of the self element to a certain value
    pub fn set(mut self, attribute: Attribute, value: AttributeValue) -> Self {
        self.attributes.insert(attribute, value.clone());
        self
    }

    /// Gets an immutable reference to the attributes of this Element
    pub fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Gets an immutable reference to the children of this Element
    pub fn get_children(&self) -> &Children {
        &self.children
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        let mut elem = Element::new(self.tag_name);
        for (key, value) in self.attributes.iter() {
            elem.attributes.insert(*key, value.clone());
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
