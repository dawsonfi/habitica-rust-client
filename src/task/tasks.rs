use serde_json::Value;

pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    pub fn new(raw_tasks: Value) -> Tasks {
        let task_list = raw_tasks.get("data").unwrap()
            .as_array().unwrap()
            .iter()
            .map(Task::new)
            .collect();

        Tasks { tasks: task_list }
    }
}

pub struct Task {

    text: String

}

impl Task {

    pub fn new(raw_task: &Value) -> Task {
        Task { text: Task::get_value_string(raw_task, "text") }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    fn get_value_string(raw_task: &Value, index: &str) -> String {
        raw_task.get(index)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
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
