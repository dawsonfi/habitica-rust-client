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

    #[test]
    fn should_build_tasks_from_response() {
        let data = r#"{
                                "data": [
                                    {"text": "Todo"}
                                ]
                             }"#;
        let raw_tasks = serde_json::from_str(data).unwrap();

        let tasks = Tasks::new(raw_tasks);

        assert_eq!(tasks.tasks.len(), 1);
    }

    #[test]
    fn should_build_task_from_value() {
        let data = r#"{ "text": "Todo" }"#;
        let raw_task = serde_json::from_str(data).unwrap();

        let task = Task::new(&raw_task);

        assert_eq!(task.get_text(), "Todo");
    }

}
