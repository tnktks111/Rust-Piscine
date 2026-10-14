use handling::open_or_create;
use std::fs;

#[test]
fn test_open_or_create() {
    let path = "test_handling.txt";

    // 新しいファイルを作成
    open_or_create(&path, "hello");
    assert_eq!(fs::read_to_string(path).unwrap(), "hello");

    // 既存のファイルに追記
    open_or_create(&path, " world");
    assert_eq!(fs::read_to_string(path).unwrap(), "hello world");

    // 後片付け
    fs::remove_file(path).unwrap();
}
