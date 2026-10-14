mod err;

use err::{ParseErr, ReadErr};
use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        fn invalid_field(field: &str) -> ParseErr {
            ParseErr::Malformed(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid field: {field}"),
            )))
        }

        let json_str = fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        if json_str.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let parsed = json::parse(&json_str).map_err(|e| ParseErr::Malformed(Box::new(e)))?;

        let title = parsed["title"]
            .as_str()
            .ok_or_else(|| invalid_field("title"))?
            .to_string();
        let tasks_json = &parsed["tasks"];

        if !tasks_json.is_array() {
            return Err(Box::new(invalid_field("tasks")));
        }

        if tasks_json.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks: Vec<Task> = Vec::with_capacity(tasks_json.len());

        for task in tasks_json.members() {
            let id = task["id"].as_u32().ok_or_else(|| invalid_field("id"))?;
            let description = task["description"]
                .as_str()
                .ok_or_else(|| invalid_field("description"))?
                .to_string();
            let level = task["level"]
                .as_u32()
                .ok_or_else(|| invalid_field("level"))?;

            tasks.push(Task {
                id,
                description,
                level,
            });
        }

        Ok(TodoList { title, tasks })
    }
}
