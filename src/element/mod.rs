use properties::property_value::PropertyValue;
use properties::Properties;
use tag_name::TagName;

use std::clone::Clone;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type Attributes = HashMap<Properties, PropertyValue>;

pub mod properties;
pub mod tag_name;

#[derive(Debug)]
pub struct Element {
    tag_name: TagName,
    attributes: Attributes,
    children: Vec<Element>,
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

impl Element {
    pub fn new(tag_name: TagName) -> Element {
        Element {
            tag_name,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn append(&self, child: Self) -> Self {
        let mut elem = self.clone();
        elem.children.push(child);
        elem
    }
    pub fn set(&self, attribute: Properties, value: PropertyValue) -> Self {
        let mut elem = self.clone();
        elem.attributes.insert(attribute, value.clone());
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
