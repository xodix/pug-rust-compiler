use std::env;
use std::fs;
use std::path::Path;

pub mod compiler;
pub mod tokenizer;

use utils::output_to_file;

// TODO split compilation onto multiple threads
// TODO add raw HTML

/// This is a basis for a pug compiler

fn main() {
    let arg1 = env::args().nth(1).expect("No filename given");
    let src_path = Path::new(&arg1);
    let src = get_src(src_path);

    let out = compiler::compile(tokenizer::tokenize(&src));

    output_to_file(&out, String::from("a.html")).unwrap();
}

fn get_src(path: &Path) -> Vec<u8> {
    fs::read(path).expect("Can't read given file")
}

#[cfg(test)]
mod tests;
