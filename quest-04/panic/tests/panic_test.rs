use panic::*;
use std::fs::File;

#[test]
fn test_open_file() {
    let path = std::env::temp_dir().join("rust_piscine_panic_test.txt");

    File::create(&path).unwrap();

    let file = open_file(path.to_str().unwrap());
    assert!(file.metadata().is_ok());

    std::fs::remove_file(&path).unwrap();
}

#[test]
#[should_panic]
fn test_open_file_not_found() {
    open_file("/nonexistent_directory_12345/nonexistent_file.txt");
}
