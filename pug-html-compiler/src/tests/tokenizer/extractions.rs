#[test]
fn test_attribute_extraction() {
    use crate::tokenizer::extractions::extract_attributes;

    let element = b"h1(hello=\"world\", my=\"friend\") some text";

    let extracted = extract_attributes(element);

    assert_eq!(
        extracted,
        Some("hello=\"world\" my=\"friend\"".as_bytes().to_vec())
    );
}

#[test]
fn test_text_extraction() {
    use crate::tokenizer::extractions::extract_text;

    let element = b"h1(hello=\"world\", my=\"friend\") some text";

    let extracted = extract_text(element);

    assert_eq!(extracted, Some("some text".as_bytes()));
}
