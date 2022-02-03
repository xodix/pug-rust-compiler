use crate::tokenizer::node::{Attributes, Node, NodeType};

#[test]
fn test_new_node() {
    let attributes: Attributes = Some(b"x=12".to_vec());
    let node_type = NodeType::Tag(b"this-is-a-element");
    let indent = 15;
    let text = None;

    let new_node = Node::new(node_type.clone(), indent, attributes.clone(), text);

    assert_eq!(
        new_node,
        Node {
            node_type: node_type.clone(),
            attributes,
            indent,
            text
        }
    );
}
