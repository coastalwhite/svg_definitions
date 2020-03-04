type Document = Vec<Element>;

use std::io::{Read, Write};
use std::io;

use svg::parser::Event;
use svg::node::element::tag::Type;

/// Open a document.
pub fn open<T>(path: T) -> io::Result<Document>
where
    T: AsRef<Path>,
{
    let parser = svg::open(path)?;

    let document = Vec::new();

    for event in parser {
        match event {
            Event::Tag(tag_string, Type::Start, plain_attributes) => {
                
            }
        }
    }
}

/// Open a document.
pub fn read<T>(path: T) -> io::Result<Document>
where
    T: Read,
{
    
}

/// Open a document.
pub fn save<T>(path: T, document: &Document) -> io::Result<()> where
    T: AsRef<Path>,
{
    
}

/// Open a document.
pub fn write<T>(target: T, document: &Document) -> io::Result<()> where
    T: Write,
{
    
}