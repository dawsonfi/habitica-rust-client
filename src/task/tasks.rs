pub struct Tasks {

    tasks: Vec<Task>

}

pub struct Task {

}

impl Tasks {

    pub fn new(tasks: Vec<Task>) -> Tasks {
        Tasks { tasks }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_build_tasks_with_collection_of_task() {

        let task = Task {};

        let tasks = Tasks::new(vec![task]);

        assert_eq!(tasks.tasks.len(), 1);
    }

}