pub mod extractions;
pub mod node;
pub mod node_types;

use node::Node;
use node_types::decide_element_type;

use utils::{find_end_of_line, remove_commas};

use self::{
    extractions::{extract_attributes, extract_text},
    node::NodeType,
};

/// ascii representation of a horizontal tab (basis of indentation and scope)
const TAB: u8 = 9;
const SPACE: u8 = 32;

/// ascii representation of new line
const NEW_LINE: u8 = b'\n';

/// parsing text into Nodes
pub fn tokenize(src: &Vec<u8>) -> Vec<Node> {
    let mut elements = Vec::new();
    // state of scope in current iteration
    let mut indent_level = 0u32;

    // holds pointer to evaluated character
    let mut exec_pos = 0usize;

    while exec_pos < src.len() {
        let ch = src[exec_pos];

        match ch {
            // tab or space extends scope
            TAB | SPACE => {
                indent_level += 1;
            }

            // new line clears local state of indentation
            NEW_LINE => {
                indent_level = 0;
            }

            // check if character is not whitespace 32 = space
            33.. => {
                let element_end = exec_pos + find_end_of_line(&src[exec_pos..]) - 1;
                let line = &src[exec_pos..element_end];

                let node_type = decide_element_type(line);
                if let NodeType::Tag(element) = node_type {
                    let attributes = extract_attributes(&remove_commas(element).clone());
                    let text = extract_text(element);

                    elements.push(Node::new(node_type, indent_level, attributes, text));
                } else if let NodeType::SelfClosingTag(element) = node_type {
                    let attributes = extract_attributes(element);
                    let text = extract_text(element);

                    elements.push(Node::new(node_type, indent_level, attributes, text));
                } else {
                    elements.push(Node::new(node_type, indent_level, None, None));
                }

                exec_pos = element_end - 1;
            }

            _ => (),
        }

        exec_pos += 1;
    }

    elements
}
