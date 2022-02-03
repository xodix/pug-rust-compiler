use std::borrow::Cow;

#[cfg(test)]
mod tests;

/**
determines if slice a begins with contents of slice b

example:
```
let a = &[1, 2, 3, 4, 5, 6];
let b = &[1, 2, 3];

assert_eq!(slice_begins_with(a, b), true);
```
*/
pub fn slice_begins_with<T: std::cmp::PartialEq>(a: &[T], b: &[T]) -> bool {
    if b.len() > a.len() {
        return false;
    }

    if &a[..b.len()] == b {
        true
    } else {
        false
    }
}

/**
searches for element in a slice and returns its index

example:
```
let slice = &[1, 2, 3, 4, 5, 6];
let element = 3;

assert_eq!(find_element(slice, element), Some(2));
```
*/
pub fn find_element<T: std::cmp::PartialEq>(haystack: &[T], needle: T) -> Option<usize> {
    for (i, element) in haystack.iter().enumerate() {
        if needle == *element {
            return Some(i);
        }
    }

    None
}

/**
remove commas from a slice

example:
```
let slice = &[1, 2, 2, 2];

assert_eq!(remove_commas(slice), Vec::new());
```
*/
pub fn remove_commas(slice: &[u8]) -> Vec<u8> {
    let mut stripped_args = Vec::new();

    for &ch in &slice[1..] {
        if ch != b',' {
            stripped_args.push(ch);
        }
    }

    stripped_args
}

/**
find the end of element type in slice

example:
```
let slice = &[1, 2, 3, 10, 5, 3];

assert_eq!(find_end_of_line(slice), 3);
```
*/
pub fn find_end_of_line(src: &[u8]) -> usize {
    /// ascii representation of new line
    const NEW_LINE: u8 = b'\n';

    for (i, ch) in src.iter().enumerate() {
        if *ch == NEW_LINE {
            return i;
        }
    }

    src.len() - 1
}

/**
Vec<u8> to String conversion

example:
```
let vec = vec![10, 46, 55, 55, 46];

assert_eq!(output_to_string(&vec).as_ref(), "\n.77."));
```
*/
pub fn output_to_string<'a>(out: &'a Vec<u8>) -> Cow<'a, str> {
    String::from_utf8_lossy(out)
}

/**
writing output of compiler to file

example:
```
let output = vec![10, 46, 55, 55, 46];

output_to_file(output, String::from("result.html")).unwrap();
```
*/
pub fn output_to_file(out: &Vec<u8>, path: String) -> Result<(), std::io::Error> {
    use std::fs;
    fs::write(path, out)?;

    Ok(())
}

/**
Flat prints reads value as if it was flattened without modifying or copying the source variable
and separates output with given delimiter.

example:
```
let source = vec![vec![1, 2, 3, 4], vec![1, 2, 3]]

read_flat(source, ",")

// prints 1,2,3,4,1,2,3
```
*/
pub fn print_flat<T>(source: Vec<Vec<T>>, delimiter: &str) {
    for v in source {
        for elem in 0..v.len() {
            print!("{}{}", elem, delimiter)
        }
    }
}
