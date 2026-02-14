use std::fs;
use std::path::PathBuf;

pub fn load_fixture(filename: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(filename);

    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read fixture: {:?}", path))
}

#[allow(dead_code)]
pub fn load_book_config() -> String {
    load_fixture("book_config.json")
}

#[allow(dead_code)]
pub fn load_chapter_content() -> String {
    load_fixture("chapter_content.json")
}
