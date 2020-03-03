use std::hash::{Hash, Hasher};

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct IdentifierProps {
    inner_string: String,
}

// Implementation of IdentifierProps
impl IdentifierProps {
    pub fn new(inner: &str) -> Result<IdentifierProps, usize> {
        for (index, character) in inner.as_bytes().iter().enumerate() {
            if !IdentifierProps::is_allowed_character(character.clone()) {
                return Err(index);
            }
        }

        Ok(IdentifierProps {
            inner_string: String::from(inner),
        })
    }

    /// Allow numbers, alphabetic characters and seperators
    fn is_allowed_character(character: u8) -> bool {
        (character >= b'0' && character <= b'9')
            || (character >= b'a' && character <= b'z')
            || (character >= b'A' && character <= b'Z')
            || character == b'-'
            || character == b'_'
            || character == b' '
    }
}

impl ToString for IdentifierProps {
    fn to_string(&self) -> String {
        self.inner_string.clone()
    }
}

impl Hash for IdentifierProps {
    fn hash<T: Hasher>(&self, state: &mut T) {
        self.inner_string.hash(state);
    }
}
