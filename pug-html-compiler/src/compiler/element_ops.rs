use utils::find_element;

const OPENING_BRACKET: u8 = b'<';
const CLOSING_BRACKET: u8 = b'>';
const SLASH: u8 = b'/';

/// get element type and convert it to opening html tag "html" -> "<html>"
pub fn open_tag(element: &[u8], attributes: &Option<Vec<u8>>) -> Vec<u8> {
    let mut opening = Vec::new();

    opening.push(OPENING_BRACKET);

    opening.extend_from_slice(&element);

    if let Some(attrs) = attributes {
        opening.push(b' ');
        opening.extend(attrs);
    }

    opening.push(CLOSING_BRACKET);

    opening
}

/// get element type and convert it to closing html tag "html" -> "</html>"
pub fn close_tag(element: &[u8]) -> Vec<u8> {
    let mut closing = Vec::new();

    closing.push(OPENING_BRACKET);
    closing.push(SLASH);

    if let Some(end) = find_element(element, b'(') {
        closing.extend_from_slice(&element[..end]);
    } else if let Some(end) = find_element(element, b' ') {
        closing.extend_from_slice(&element[..end]);
    } else {
        closing.extend_from_slice(element);
    }

    closing.push(CLOSING_BRACKET);

    closing
}
