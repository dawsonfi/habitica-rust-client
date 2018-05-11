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
    /// let tasks = habitica_client.get_tasks();
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
    /// print!("{:?}", task.get_text()));
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

    #[test]
    fn should_build_tasks_from_response() {
        let raw_tasks = json!({"data": [{"text": "Todo"}]});

        let tasks = Tasks::new(raw_tasks);

        assert_eq!(tasks.tasks.len(), 1);
    }

    #[test]
    fn should_build_task_from_value() {
        let raw_task = json!({"text": "Todo"});

        let task = Task::new(&raw_task);

        assert_eq!(task.get_text(), "Todo");
    }

}
