pub enum ParseError {
    RoxmltreeError(roxmltree::Error),
    AttributeNotFound(String),
    TagNotFound(String),
}

pub fn string_to_tag(string: String) -> Option<crate::tag_name::TagName> {

}

pub fn string_to_attribute(string: String) -> Option<crate::attributes::Attribute> {
    
}

pub fn node_to_element(root: roxmltree::Node) -> Result(crate::Element, ParseError) {
    let 
    root.
}

pub fn parse_text(xml: &str) -> Result(crate::Element, ParseError) {
    let doc = roxmltree::Document::parse(xml).map_err(|err| ParseError::RoxmltreeError(err))?;

    doc.root_element()
}
