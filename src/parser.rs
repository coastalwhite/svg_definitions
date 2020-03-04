use std::collections::HashMap;

use crate::tag_name;
use crate::attributes::Attribute;
use crate::attributes;
use crate::Element;

type ParseError = (usize, String);

pub fn parse(text: &str) -> Result<Vec<Element>, ParseError> {
    let plain_text = remove_whitespace(text);
    let elements = Vec::<Element>::new();
    let parents = Vec::<Element>::new();

    let mut chars = plain_text.chars();

    let mut index = 0;
    while index < plain_text.len() {
        let character = chars.next();
        match character {
            Some('<') => {
                let inner_tag = get_inner_tag(&plain_text[index+1..], index)?;
                let parsed = parse_inner_tag(&inner_tag[..], index+1)?;

                let cur_parent = parents.iter().last();

                if parsed.is_closing {
                    if parsed.is_standalone {
                        match cur_parent {
                            None => elements.push(parsed.into()),
                            Some(_) => {
                                let p = parents.iter_mut().last().unwrap();
                                p = p.append(parsed.into());
                            } 
                        }
                    } else {
                        
                    }
                }
            }
            _ => ()
        }
    }

    Err((0, "".into()))
}

#[derive(Debug)]
struct TempElement {
    tag: tag_name::TagName,
    attributes: HashMap<Attribute, String>,
    is_closing: bool,
    is_standalone: bool
}

impl Into<Element> for TempElement {
    fn into(self) -> Element {
        Element {
            tag_name: self.tag,
            attributes: self.attributes,
            children: vec![],
            inner: String::from("")
        }
    }
}

fn get_inner_tag(text: &str, start: usize) -> Result<String, ParseError> {
    let mut inside_quotes = false;
    let mut is_escaped = false;

    let inner_tag: String = text.chars().take_while(|c| {
        match c {
            '>' => if !inside_quotes { return false; },
            '\\' => is_escaped = !is_escaped,
            '"' => if !is_escaped {
                inside_quotes = !inside_quotes;
            },
            _ => ()
        }

        if c != &'\\' {
            is_escaped = false
        }

        true
    }).collect();

    if inner_tag.len() == text.len() {
        return Err((start, format!("Failed to close tag at: {}", start)));
    }

    Ok(inner_tag)
}

fn parse_inner_tag(text: &str, start: usize) -> Result<TempElement, ParseError> {
    let attributes_strs = split_attributes(text);
    
    if attributes_strs.len() == 0 {
        return Err((start, format!("No tag content at: {}", start)));
    }

    let mut tag_string = &attributes_strs.get(0).unwrap()[..];

    let is_closing = match tag_string.chars().next() {
        Some('/') => true,
        _ => false
    };

    let is_standalone = false;

    if is_closing {
        tag_string = &tag_string[1..];
    }

    let tag = tag_name::parse(tag_string)
        .map_err(|_| (start, format!("Failed to parse tag '{}' at: {}", tag_string, start)))?;

    let mut attributes = HashMap::new();

    if is_closing {
        return Ok(TempElement { tag, attributes, is_closing, is_standalone } );
    }

    let mut index = start + &tag_string.len() + 1;
    for attribute_string in attributes_strs.iter().skip(1) {
        if index == text.len() + start - 1 && text.chars().skip(text.len()-1).next() == Some('/') {
            is_standalone = true;
        } else {
            let parsed = parse_attribute(attribute_string, index)?;
            index += attribute_string.len() + 1;

            attributes.insert(parsed.0, parsed.1);
        }
    }

    Ok(TempElement { tag, attributes, is_closing, is_standalone } )
}

fn parse_attribute(text: &str, start: usize) -> Result<(Attribute, String), ParseError> {
    let attribute_string: String = text.chars().take_while(|c| c != &'=').collect();

    let attribute = attributes::parse(&attribute_string[..])
        .map_err(|_| (start, format!("Failed to parse attribute '{}' at: {}", attribute_string, start)))?;

    let value_start = attribute_string.len()+1;
    let mut value = &text[(value_start)..];

    if value.len() == 0 {
        return Err((start + value_start, format!("Missing value of {} at: {}", attribute.to_string(), start + value_start)));
    }

    if value.chars().next() == Some('"') {
        value = &value[1..value.len()-1];
    }

    Ok((attribute, String::from(value)))
}

// https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-string
fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace() && c != &' ').collect()
}

fn sum_lengths(vec: &Vec<String>) -> usize {
    let mut size = 0;

    for s in vec.iter() {
        size += s.len();
    }

    size
}

fn split_attributes(s: &str) -> Vec<String> {
    let mut attributes = Vec::new();
    let mut is_escaped = false;
    let mut closed_value = false;
    let mut inside_quotes = false;

    attributes.push(s.chars().take_while(|c| c != &' ').collect());

    let mut index = sum_lengths(&attributes) + attributes.len();
    while index < s.len() {
        attributes.push(s[index..].chars().take_while(|c| {
            match c {
                ' ' => if closed_value || !inside_quotes { return false; },
                '\\' => { is_escaped = !is_escaped },
                '"' => {
                    if !is_escaped {
                        if inside_quotes {
                            closed_value = true;
                            inside_quotes = false;
                        } else {
                            inside_quotes = true;
                        }
                    }
                },
                _ => (),
            }

            if c != &'\\' {
                is_escaped = false
            }

            true
        }).collect());

        is_escaped = false;
        closed_value = false;
        index = sum_lengths(&attributes) + attributes.len();
    }
    
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_attributes() {
        assert_eq!(
            vec![
                String::from("svg"),
                String::from("href=\"xx x\""),
                String::from("dd=\"\""),
                String::from("aa=\"\""),
                String::from("gg=\"\""),
                String::from("x=yzwz")
            ],
            split_attributes("svg href=\"xx x\" id=\"\" stroke=\"\" fill=\"\" x=1")
        );
        assert_eq!(vec![String::from("svg"), String::from("href=\"xx\\\"x\"")], split_attributes("svg href=\"xx\\\"x\""));
    }

    #[test]
    fn test_parse() {
        println!("{:?}",parse_inner_tag("svg href=\"xx x\" id=\"\" stroke=\"\" fill=\"\" x=1 /", 0));
        assert!(false);
    }
}