#[test]
fn test_slice_begins_with() {
    use crate::slice_begins_with;
    let a = b"hi there";
    let b = b"hi";

    assert_eq!(slice_begins_with(a, b), true);
}

#[test]
fn test_find_element() {
    use crate::find_element;
    let haystack = b"hello  world";
    let needle = b'o';

    assert_eq!(find_element(haystack, needle), Some(4));
}

#[test]
fn test_remove_whitespace() {
    use crate::remove_commas;
    let element = b"(hello = 'world', hell, on, earth)";

    assert_eq!(remove_commas(element), b"hello = 'world' hell on earth)");
}

#[test]
fn test_output_to_str() {
    use crate::output_to_string;

    let hello_arr = b"hello".to_vec();
    let hello = "hello".to_string();

    let fn_output = output_to_string(&hello_arr);

    assert_eq!(fn_output, hello);
}

#[test]
fn test_output_to_file() -> Result<(), std::io::Error> {
    use crate::output_to_file;

    use std::fs;

    let contents = b"<html></html>".to_vec();
    let path = "test.html".to_string();

    output_to_file(&contents, path.clone())?;

    let res = std::fs::read(path.clone())? == contents;

    fs::remove_file(path)?;

    assert!(res);

    Ok(())
}
