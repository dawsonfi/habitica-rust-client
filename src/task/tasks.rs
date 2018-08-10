use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct Task {
    text: Option<String>,
    frequency: Option<String>,
    task_type: Option<String>,
    notes: Option<String>,
    repeat: Option<TaskRepeat>,
    every_x: Option<i64>,
    next_due: Option<Vec<String>>,
    completed: Option<bool>,
    is_due: Option<bool>,
    checklist: Option<TaskCheckList>
}

#[derive(Debug)]
pub struct TaskRepeat {
    days: HashMap<String, bool>
}

#[derive(Debug)]
pub struct TaskCheckList {
    list: Vec<TaskCheckListItem>,
}

#[derive(Debug)]
pub struct TaskCheckListItem {
    completed: bool,
    text: String
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
            frequency: Task::get_value_string(raw_task, "frequenci"),
            task_type: Task::get_value_string(raw_task, "type"),
            notes: Task::get_value_string(raw_task, "notes"),
            repeat: Some(TaskRepeat { days: HashMap::new() }),
            every_x: Task::get_value_i64(raw_task, "everyX"),
            next_due: Some(vec![]),
            completed: Task::get_value_bool(raw_task, "completed"),
            is_due: Task::get_value_bool(raw_task, "isDue"),
            checklist: Some(TaskCheckList { list: vec![] })
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
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    pub fn get_frequency(&self) -> &Option<String> {
        &self.frequency
    }

    pub fn get_task_type(&self) -> &Option<String> {
        &self.task_type
    }

    pub fn get_notes(&self) -> &Option<String> {
        &self.notes
    }

    pub fn get_repeat(&self) -> &Option<TaskRepeat> {
        &self.repeat
    }

    pub fn get_every_x(&self) -> &Option<i64> {
        &self.every_x
    }

    pub fn get_next_due(&self) -> &Option<Vec<String>> {
        &self.next_due
    }

    pub fn is_completed(&self) -> &Option<bool> {
        &self.completed
    }

    pub fn is_due(&self) -> &Option<bool> {
        &self.is_due
    }

    pub fn get_checklist(&self) -> &Option<TaskCheckList> {
        &self.checklist
    }

    fn get_value_string(raw_task: &Value, index: &str) -> Option<String> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() { Some(raw_value.unwrap().as_str().unwrap().to_string()) } else { None }
    }

    fn get_value_i64(raw_task: &Value, index: &str) -> Option<i64> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() { raw_value.unwrap().as_i64() } else { None }
    }

    fn get_value_bool(raw_task: &Value, index: &str) -> Option<bool> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() { raw_value.unwrap().as_bool() } else { None }
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
    fn should_build_daily_task_from_value() {
        let data = get_tasks_response_data();
        let raw_tasks: Value = serde_json::from_str(&data).unwrap();
        let raw_task = raw_tasks
            .get("data").unwrap()
            .as_array().unwrap()
            .get(3).unwrap();

        let task = Task::new(&raw_task);

        assert_eq!(task.get_text(), &Some("Example TODO".to_string()));
//        assert_eq!(task.get_frequency(), "daily");
//        assert_eq!(task.get_task_type(), "todo");
//        assert_eq!(task.get_notes(), "https://w.amazon.com/bin/view/EE/Learn/Resources/SDEToolkit/Development_Plan/");
//        assert_eq!(task.get_every_x(), &1i64);
//        assert_eq!(task.get_next_due(), &vec!["String".to_string()]);
//        assert_eq!(task.is_completed(), &false);
//        assert_eq!(task.is_due(), &true);
//        assert_eq!(task.get_repeat(), TaskRepeat { days: HashMap::new() });
//        assert_eq!(task.get_checklist(), TaskCheckList { list: vec![TaskCheckListItem { completed: false, text: "CheckList 1".to_string() }]} );
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
