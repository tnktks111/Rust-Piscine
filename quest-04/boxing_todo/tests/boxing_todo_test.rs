use boxing_todo::{Task, TodoList};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

// テストごとに異なるファイルパスを生成
struct TempJson {
    path: PathBuf,
}

impl TempJson {
    fn new(content: Option<&str>) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        let path =
            std::env::temp_dir().join(format!("boxing_todo_{}_{}.json", std::process::id(), id));

        if let Some(content) = content {
            fs::write(&path, content).unwrap();
        }

        Self { path }
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

// テスト終了時にファイルを削除
impl Drop for TempJson {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[test]
fn test_valid_todo() {
    let file = TempJson::new(Some(
        r#"{
            "title": "My Todo List",
            "tasks": [
                {
                    "id": 0,
                    "description": "do this",
                    "level": 0
                },
                {
                    "id": 1,
                    "description": "do that",
                    "level": 5
                }
            ]
        }"#,
    ));

    let result = TodoList::get_todo(file.path()).unwrap();

    let expected = TodoList {
        title: "My Todo List".to_string(),
        tasks: vec![
            Task {
                id: 0,
                description: "do this".to_string(),
                level: 0,
            },
            Task {
                id: 1,
                description: "do that".to_string(),
                level: 5,
            },
        ],
    };

    assert_eq!(result, expected);
}

#[test]
fn test_empty_file() {
    let file = TempJson::new(Some(""));

    let err = TodoList::get_todo(file.path()).unwrap_err();

    assert_eq!(err.to_string(), "Failed to parse todo file");
}

#[test]
fn test_empty_tasks() {
    let file = TempJson::new(Some(
        r#"{
            "title": "Empty Todo",
            "tasks": []
        }"#,
    ));

    let err = TodoList::get_todo(file.path()).unwrap_err();

    assert_eq!(err.to_string(), "Failed to parse todo file");
}

#[test]
fn test_invalid_json() {
    let file = TempJson::new(Some(
        r#"{
            "title": "Todo",
            "tasks": [
        }"#,
    ));

    let err = TodoList::get_todo(file.path()).unwrap_err();

    assert_eq!(err.to_string(), "Failed to parse todo file");
}

#[test]
fn test_invalid_fields() {
    let cases = [
        // title が存在しない
        r#"{"tasks":[{"id":0,"description":"test","level":1}]}"#,
        // title が文字列ではない
        r#"{"title":123,"tasks":[{"id":0,"description":"test","level":1}]}"#,
        // tasks が配列ではない
        r#"{"title":"Todo","tasks":"invalid"}"#,
        // id が文字列
        r#"{"title":"Todo","tasks":[{"id":"abc","description":"test","level":1}]}"#,
        // description が存在しない
        r#"{"title":"Todo","tasks":[{"id":0,"level":1}]}"#,
        // level が負数
        r#"{"title":"Todo","tasks":[{"id":0,"description":"test","level":-1}]}"#,
    ];

    for content in cases {
        let file = TempJson::new(Some(content));

        assert!(
            TodoList::get_todo(file.path()).is_err(),
            "Expected error for: {content}"
        );
    }
}

#[test]
fn test_file_not_found() {
    let file = TempJson::new(None);

    let err = TodoList::get_todo(file.path()).unwrap_err();

    assert_eq!(err.to_string(), "Failed to read todo file");
}
