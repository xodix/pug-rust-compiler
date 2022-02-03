use crate::tokenizer::node::NodeType;
use crate::tokenizer::node_types::{self, decide_element_type};
#[test]
fn test_doctype_recognition() {
    let doctype = b"doctype";
    let element_type = decide_element_type(doctype);

    assert_eq!(element_type, NodeType::Doctype);
}

#[test]
fn test_invisible_comment_recognition() {
    let comment = b"// this is a comment";
    let element_type = decide_element_type(comment);

    assert_eq!(element_type, NodeType::InvisibleComment);
}

#[test]
fn test_visible_comment_recognition() {
    let comment = b"//- this is a comment";
    let element_type = decide_element_type(comment);

    assert_eq!(element_type, NodeType::Comment(b" this is a comment"));
}

#[test]
fn test_plain_text() {
    let text = b"| this is a text pipe";
    let element_type = decide_element_type(text);

    assert_eq!(element_type, NodeType::Text(&text[1..]));
}

#[test]
fn test_normal_tag() {
    let element = b"hello()";
    let element_type = decide_element_type(element);

    assert_eq!(element_type, NodeType::Tag(element));
}

#[test]
fn test_self_closing_tag() {
    let element = b"img(hello='world')";
    let element_type = decide_element_type(element);

    assert_eq!(element_type, NodeType::SelfClosingTag(element));
}

#[test]
fn test_is_self_closing() {
    let element = b"img";
    let is_self_closing = node_types::is_self_closing(element);

    assert_eq!(is_self_closing, true);
}
