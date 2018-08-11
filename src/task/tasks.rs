use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Tasks {
    tasks: Vec<Task>,
}

#[derive(Debug, PartialEq)]
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
    checklist: Option<TaskCheckList>,
}

#[derive(Debug, PartialEq)]
pub struct TaskRepeat {
    days: HashMap<String, bool>,
}

#[derive(Debug, PartialEq)]
pub struct TaskCheckList {
    list: Vec<TaskCheckListItem>,
}

#[derive(Debug, PartialEq)]
pub struct TaskCheckListItem {
    completed: bool,
    text: String,
    id: String,
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
            frequency: Task::get_value_string(raw_task, "frequency"),
            task_type: Task::get_value_string(raw_task, "type"),
            notes: Task::get_value_string(raw_task, "notes"),
            repeat: Some(TaskRepeat {
                days: Task::get_value_map(raw_task, "repeat"),
            }),
            every_x: Task::get_value_i64(raw_task, "everyX"),
            next_due: Task::get_value_vec(raw_task, "nextDue"),
            completed: Task::get_value_bool(raw_task, "completed"),
            is_due: Task::get_value_bool(raw_task, "isDue"),
            checklist: Some(TaskCheckList {
                list: Task::get_value_checklist(raw_task, "checklist"),
            }),
        }
    }

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

        return if raw_value.is_some() {
            Some(raw_value.unwrap().as_str().unwrap().to_string())
        } else {
            None
        };
    }

    fn get_value_vec(raw_task: &Value, index: &str) -> Option<Vec<String>> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() {
            Some(
                raw_value
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect(),
            )
        } else {
            None
        };
    }

    fn get_value_map(raw_task: &Value, index: &str) -> HashMap<String, bool> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() {
            raw_value
                .unwrap()
                .as_object()
                .unwrap()
                .into_iter()
                .map(|(key, value)| (String::from(key.to_owned()), value.as_bool().unwrap()))
                .collect()
        } else {
            HashMap::new()
        };
    }

    fn get_value_checklist(raw_task: &Value, index: &str) -> Vec<TaskCheckListItem> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() {
            raw_value
                .unwrap()
                .as_array()
                .unwrap()
                .into_iter()
                .map(|value| TaskCheckListItem {
                    completed: Task::get_value_bool(value, "completed").unwrap(),
                    text: Task::get_value_string(value, "text").unwrap(),
                    id: Task::get_value_string(value, "id").unwrap(),
                })
                .collect()
        } else {
            Vec::new()
        };
    }

    fn get_value_i64(raw_task: &Value, index: &str) -> Option<i64> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() {
            raw_value.unwrap().as_i64()
        } else {
            None
        };
    }

    fn get_value_bool(raw_task: &Value, index: &str) -> Option<bool> {
        let raw_value = raw_task.get(index);

        return if raw_value.is_some() {
            raw_value.unwrap().as_bool()
        } else {
            None
        };
    }
}

impl TaskRepeat {
    pub fn get_days(&self) -> &HashMap<String, bool> {
        &self.days
    }
}

impl TaskCheckList {
    pub fn get_list(&self) -> &Vec<TaskCheckListItem> {
        &self.list
    }
}

impl TaskCheckListItem {
    pub fn is_completed(&self) -> &bool {
        &self.completed
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_id(&self) -> &String {
        &self.id
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
            .get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .get(3)
            .unwrap();

        let task = Task::new(&raw_task);

        let repeat = get_task_repeat();
        let check_list = TaskCheckList {
            list: vec![get_task_check_list_item()],
        };

        assert_eq!(task.get_text(), &Some("Example TODO".to_string()));
        assert_eq!(task.get_frequency(), &Some("daily".to_string()));
        assert_eq!(task.get_task_type(), &Some("todo".to_string()));
        assert_eq!(
            task.get_notes(),
            &Some(
                "https://w.amazon.com/bin/view/EE/Learn/Resources/SDEToolkit/Development_Plan/"
                    .to_string()
            )
        );
        assert_eq!(task.get_every_x(), &Some(1i64));
        assert_eq!(
            task.get_next_due(),
            &Some(vec![
                "2018-06-05T03:00:00.000Z".to_string(),
                "2018-06-06T04:00:00.000Z".to_string(),
            ])
        );
        assert_eq!(task.is_completed(), &Some(false));
        assert_eq!(task.is_due(), &Some(true));
        assert_eq!(task.get_repeat(), &Some(TaskRepeat { days: repeat }));
        assert_eq!(task.get_checklist(), &Some(check_list));
    }

    #[test]
    fn should_return_days() {
        let task_repeat = TaskRepeat {
            days: get_task_repeat(),
        };

        assert_eq!(task_repeat.get_days(), &get_task_repeat());
    }

    #[test]
    fn should_return_checklist() {
        let check_list = TaskCheckList {
            list: vec![get_task_check_list_item()],
        };

        assert_eq!(check_list.get_list(), &vec![get_task_check_list_item()]);
    }

    #[test]
    fn should_return_is_completed() {
        let check_list_item = get_task_check_list_item();

        assert_eq!(check_list_item.is_completed(), &false);
    }

    #[test]
    fn should_return_text() {
        let check_list_item = get_task_check_list_item();

        assert_eq!(check_list_item.get_text(), &"CheckList 1".to_string());
    }

    #[test]
    fn should_return_id() {
        let check_list_item = get_task_check_list_item();

        assert_eq!(
            check_list_item.get_id(),
            &"a1cdd8c9-8012-4fe6-99c1-a50db3cb5926".to_string()
        );
    }

    fn get_tasks_response_data() -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/get_tasks_response.json");

        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        data
    }

    fn get_task_repeat() -> HashMap<String, bool> {
        [
            ("m".to_string(), true),
            ("t".to_string(), false),
            ("w".to_string(), false),
            ("th".to_string(), false),
            ("f".to_string(), false),
            ("s".to_string(), false),
            ("su".to_string(), false),
        ].iter()
            .cloned()
            .collect()
    }

    fn get_task_check_list_item() -> TaskCheckListItem {
        TaskCheckListItem {
            completed: false,
            text: "CheckList 1".to_string(),
            id: "a1cdd8c9-8012-4fe6-99c1-a50db3cb5926".to_string(),
        }
    }

}
