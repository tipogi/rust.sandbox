use std::{fs, cell::RefCell, path::Path};

use chrono::Datelike;
use serde_json::{Map, Value, json};

use crate::{task::Task, status::Status};

pub struct File {}

impl File {
    pub fn read_task_file() -> Vec<Task> {
        let task_file = fs::read_to_string("files/import/task.json");
        serde_json::from_str(&task_file.unwrap()).unwrap()
    }

    pub fn read_week_task_status_file(path: &str) -> Map<String, Value> {
        let task_file = fs::read_to_string(path).unwrap();
        let parsed: Value = serde_json::from_str(&task_file).unwrap();
        parsed.as_object().unwrap().clone()
    }

    pub fn save_daily_tasks(daily_tasks: &RefCell<Vec<Status>>) {
        let mut map = Map::new();
        let now = chrono::offset::Local::now();
        let week_day = now.weekday();
        // Take the ownership of the RefCel value 
        // and add the week day key to the task entries
        map.insert(week_day.to_string(), json!(daily_tasks.take()));
        // Create the path to save the file
        let path = format!(
            "files/export/{}/week_{}.json", 
            now.year(), 
            now.iso_week().week()
        );
        File::save_file(&path, &mut map);
    }

    pub fn save_file(path: &str, map: &mut Map<String, Value>) {
        let mut file_exists = false;
        // Check if the file and folders exists before add the new entries
        match Path::new(path).exists() {
            false   => File::create_file_and_parents(path),
            _       => file_exists = true
        }
        let new_status_map = match file_exists {
            false   => map.clone(),
            true    => add_previous_days_tasks(path, map)
        };
        // Convert our Map object into String to save as a JSON file
        let contents = Value::from(new_status_map.clone()).to_string();
        fs::write(path, contents).unwrap();
    }

    pub fn create_file_and_parents(path: &str) {
        let native_path = std::path::Path::new(path);
        let prefix = native_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
    }
}

fn add_previous_days_tasks(path: &str, map: &mut Map<String, Value>) -> Map<String, Value> {
    let mut week_status_file = File::read_week_task_status_file(path);
    week_status_file.append(map);
    week_status_file
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn get_week_number() {
        let daily_tasks:RefCell<Vec<Status>> = RefCell::new(Vec::new());
        File::save_daily_tasks(&daily_tasks);
    }
}