use utils;

fn compile(src: Vec<u8>) {
    let mut indent_level = 0;
    let mut exec_pos = 0;

    while exec_pos < src.len() {
        let ch = src[exec_pos];

        match ch {
            _ if utils::slice_begins_with(&src[exec_pos..], b"if") => {
                println!(
                    "{:?}",
                    utils::output_to_string(&src[exec_pos + 2..].to_vec())
                );
            }

            b'\n' => indent_level = 0,

            b'\t' => indent_level = 0,
            b' ' => indent_level += 1,
            _ => (),
        };
    }
}
