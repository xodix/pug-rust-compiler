#[derive(PartialEq, Debug, Clone)]
pub enum NodeType<'a> {
    Doctype,
    Comment(&'a [u8]),
    InvisibleComment,
    Tag(&'a [u8]),
    SelfClosingTag(&'a [u8]),
    Text(&'a [u8]),
    Code(&'a [u8]),
    Mixin(&'a [u8]),
}

pub type Attributes<'a> = Option<Vec<u8>>;

#[derive(PartialEq, Debug)]
pub struct Node<'a> {
    pub node_type: NodeType<'a>,
    pub attributes: Attributes<'a>,
    pub text: Option<&'a [u8]>,
    pub indent: u32,
}

impl<'a> Node<'a> {
    pub fn new(
        node_type: NodeType<'a>,
        indent: u32,
        attributes: Attributes<'a>,
        text: Option<&'a [u8]>,
    ) -> Self {
        Self {
            node_type,
            attributes,
            indent,
            text,
        }
    }
}
