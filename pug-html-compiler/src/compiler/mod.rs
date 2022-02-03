pub mod element_ops;

use crate::tokenizer::node::{Node, NodeType};
use utils::find_element;

use self::element_ops::{close_tag, open_tag};

pub fn compile(ast: Vec<Node>) -> Vec<u8> {
    let mut fin = Vec::new();

    let mut indent_stack: Vec<&Node> = Vec::new();
    let mut last_indent = 0;

    for node in ast.iter() {
        if node.indent <= last_indent && indent_stack.len() != 0 {
            let mut curr_node = indent_stack[indent_stack.len() - 1];

            while curr_node.indent >= node.indent {
                fin.extend(close_node(curr_node));
                indent_stack.pop();

                if indent_stack.len() == 0 {
                    break;
                } else {
                    curr_node = indent_stack[indent_stack.len() - 1];
                }
            }
        }
        fin.extend(open_node(node));

        indent_stack.push(node);
        last_indent = node.indent;
    }

    for node in indent_stack.iter().rev() {
        fin.extend(close_node(node));
    }

    fin
}

pub fn open_node(node: &Node) -> Vec<u8> {
    return match node.node_type {
        NodeType::Doctype => b"<!DOCTYPE html>".to_vec(),

        NodeType::Comment(comment) => {
            let mut res = b"<!--".to_vec();

            res.extend(comment);
            res.extend(b"-->");

            res
        }

        NodeType::Tag(element) => {
            let mut res = vec![];

            if let Some(end) = find_element(element, b'(') {
                res.extend(open_tag(&element[..end], &node.attributes));
            } else if let Some(end) = find_element(element, b' ') {
                res.extend(open_tag(&element[..end], &node.attributes));
            } else {
                res.extend(open_tag(element, &node.attributes));
            }

            if let Some(text) = node.text {
                res.extend(text);
            }

            res
        }

        NodeType::SelfClosingTag(element) => {
            let mut res = Vec::with_capacity(element.len() + 4);

            res.push(b'<');

            res.extend(element);

            if let Some(attrs) = &node.attributes {
                res.extend(attrs);
            }

            res.extend(b" />");

            res
        }

        NodeType::Text(element) => element[2..].to_vec(),

        _ => Vec::new(),
    };
}

pub fn close_node(node: &Node) -> Vec<u8> {
    return match node.node_type {
        NodeType::Tag(e) => close_tag(e),
        _ => Vec::new(),
    };
}
