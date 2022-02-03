use crate::tokenizer::node::NodeType::{self, *};

const SLASH: u8 = b'/';

/// determine the type of given element
pub fn decide_element_type(element: &[u8]) -> NodeType {
    return match element {
        b"doctype" => Doctype,
        e if element[0] == SLASH && element[1] == SLASH => {
            if element[2] == b'-' {
                Comment(&e[3..])
            } else {
                InvisibleComment
            }
        }

        text if element[0] == b'|' => Text(&text[1..]),

        e => {
            if is_self_closing(e) {
                SelfClosingTag(e)
            } else {
                Tag(e)
            }
        }
    };
}

/// determine if tag is self closing
pub fn is_self_closing(element: &[u8]) -> bool {
    use utils::slice_begins_with;

    let self_closing_tags: Vec<&[u8]> = vec![
        b"area",
        b"base",
        b"br",
        b"col",
        b"embed",
        b"hr",
        b"img",
        b"input",
        b"link",
        b"meta",
        b"param",
        b"source",
        b"track",
        b"wbr",
        b"command",
        b"keygen",
        b"menuitem",
    ];

    for tag in self_closing_tags {
        if slice_begins_with(element, tag) {
            return true;
        }
    }

    false
}
