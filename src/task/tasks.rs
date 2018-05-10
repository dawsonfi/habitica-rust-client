use serde_json::Value;

pub struct Tasks {

    tasks: Vec<Task>

}

pub struct Task {

}

impl Tasks {

    pub fn new(raw_tasks: Value) -> Tasks {
        Tasks { tasks: Vec::new() }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_build_tasks_with_collection_of_task() {

        let raw_tasks = json!("{}");

        let tasks = Tasks::new(raw_tasks);

        assert_eq!(tasks.tasks.len(), 1);
    }

}