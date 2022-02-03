use crate::compiler::element_ops::{close_tag, open_tag};

#[test]
fn test_open_elem() {
    let fn_output = open_tag(b"hello", &None);
    let hello_elem = b"<hello>".to_vec();
    assert_eq!(fn_output, hello_elem);
}

#[test]
fn test_close_elem() {
    let fn_output = close_tag(b"hello");
    let hello_elem = b"</hello>".to_vec();
    assert_eq!(fn_output, hello_elem);
}
