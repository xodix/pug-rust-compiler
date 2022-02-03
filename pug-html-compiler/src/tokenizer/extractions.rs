use utils::{find_element, remove_commas};

use crate::tokenizer::node::Attributes;

pub fn extract_attributes(element: &[u8]) -> Attributes {
    if let Some(start_parse) = find_element(element, b'(') {
        let end_parse = find_element(element, b')').expect("Expected ')' when listing attributes.");

        if end_parse == start_parse + 1 {
            None
        } else {
            Some(remove_commas(&element[start_parse..end_parse]))
        }
    } else {
        None
    }
}

pub fn extract_text(element: &[u8]) -> Option<&[u8]> {
    let mut i = 0;

    while i < element.len() {
        if element[i] == b'(' {
            i += find_element(&element[i..], b')').expect("Expected ')' when listing attributes.");
        } else if element[i] == b' ' {
            return Some(&element[i + 1..]);
        }

        i += 1;
    }

    None
}
