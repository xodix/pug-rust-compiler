fn load_script(path: &std::path::Path) -> Result<Vec<u8>, std::io::Error> {
    use std::fs;

    fs::read(path)
}
