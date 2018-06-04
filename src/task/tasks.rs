use serde_json::Value;

#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct Task {
    text: String,
}

impl Tasks {
    pub fn new(raw_tasks: Value) -> Tasks {
        let task_list = raw_tasks
            .get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(Task::new)
            .collect();

        Tasks { tasks: task_list }
    }

    /// Returns all of users tasks (habits, dailies, to-dos)
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate habitica_rust_client;
    /// extern crate serde_json;
    ///
    /// use habitica_rust_client::task::tasks::Tasks;
    /// use serde_json::Value;
    ///
    /// let data = r#"{
    ///                "data": [{"text": "test"}]
    ///               }"#;
    ///
    /// let value: Value = serde_json::from_str(data).unwrap();
    /// let tasks = Tasks::new(value);
    ///
    /// print!("{:?}", tasks);
    ///
    /// ```
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

impl Task {
    pub fn new(raw_task: &Value) -> Task {
        Task {
            text: Task::get_value_string(raw_task, "text"),
        }
    }

    /// Returns the text of the task
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate habitica_rust_client;
    /// extern crate serde_json;
    ///
    /// use habitica_rust_client::task::tasks::Task;
    /// use serde_json::Value;
    ///
    /// let data = r#"{"text": "test"}"#;
    ///
    /// let value: Value = serde_json::from_str(data).unwrap();
    /// let task = Task::new(&value);
    ///
    /// print!("{:?}", task.get_text());
    /// ```
    pub fn get_text(&self) -> &String {
        &self.text
    }

    fn get_value_string(raw_task: &Value, index: &str) -> String {
        raw_task.get(index).unwrap().as_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    #[test]
    fn should_build_tasks_from_response() {
        let data = get_tasks_response_data();
        let raw_tasks: Value = serde_json::from_str(&data).unwrap();

        let tasks = Tasks::new(raw_tasks);

        assert_eq!(tasks.tasks.len(), 4);
    }

    #[test]
    fn should_build_task_from_value() {
        let data = get_tasks_response_data();
        let raw_tasks: Value = serde_json::from_str(&data).unwrap();
        let raw_task = raw_tasks
            .get("data").unwrap()
            .as_array().unwrap()
            .get(0).unwrap();

        let task = Task::new(&raw_task);

        assert_eq!(task.get_text(), "Example Habbit");
    }

    fn get_tasks_response_data() -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/get_tasks_response.json");

        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        data
    }

}
