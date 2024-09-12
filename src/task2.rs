


fn task2_a() {
    let mut task_store = TaskStore::default();

    // Create a task and add it to the task store
    let mut task = Task { name: "Test Task".to_string(), priority: 5 };
    task_store.insert_task(&task);

    // Actually I want the task's priority to be 1
    task.priority = 1;

    task_store.print_tasks();
}

#[derive(Debug)]
struct Task {
    pub name: String,
    pub priority: u32
}

#[derive(Default)]
struct TaskStore<'a> {
    tasks: Vec<&'a Task>
}
impl<'a> TaskStore<'a> {
    pub fn insert_task(&mut self, task: &'a Task) {
        self.tasks.push(task)
    }

    pub fn print_tasks(&self) {
        for task in &self.tasks {
            println!("{task:?}");
        }
    }
}






// Maybe lifetime parameters are too scary so heres a different task



fn task2_b() {
    let mut wrapper = StringWrapper::new(String::from("Initial"));

    let value_ref = wrapper.get_value(); // Immutable borrow
    println!("Value: {}", value_ref);

    wrapper.set_value(String::from("Updated")); // Mutable borrow

    println!("New Value: {}", value_ref);
}

struct StringWrapper {
    value: String,
}

impl StringWrapper {
    fn new(value: String) -> Self {
        StringWrapper { value }
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn set_value(&mut self, new_value: String) {
        self.value = new_value;
    }
}