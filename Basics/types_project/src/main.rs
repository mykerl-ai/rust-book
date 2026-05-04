#[derive(Debug)]
struct Tasks {
    title: String,
    priority: Priority,
    status: Status,
    assignee: Option<String>
}

#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High
}

#[derive(Debug)]
enum Status {
    Todo,
    InProgress,
    Done,
    Blocked(String)
}

impl Tasks {

    // fn start(self: &mut Self) {
        
    // }

    //OR

    fn start(&mut self) -> &mut Self {
        self.status = Status::InProgress;

        self
    }

    fn stop(&mut self) -> &mut Self {
        self.status = Status::Done;

        self
    }

    fn block(&mut self) -> &mut Self {
        self.status = Status::Blocked(String::from("We don't have write access to the DB"));

        self
    }




   fn print_summary(&self) {
    
    println!("\n-----------\n");
    println!("Title: {:?}", self.title);
    println!("Priority: {:?}", self.priority);
    println!("Status: {:?}", self.status);
    match &self.assignee {
        Some(profile) => println!("Assignee: {:?}", profile),
        None => println!("Assignee: Unassigned")
    };
   }
}

fn show_full_list(tasks: &Vec<Tasks>) {
    println!("INITIAL TASK LIST");

    for (i, task) in tasks.iter().enumerate() {
        println!("Task #{:#?}", i);
        task.print_summary();
    }
}

fn show_task_at_index(tasks: &Vec<Tasks>, index: usize) {
    let Some(task) = tasks.get(index) else {
        println!("Task does not exist");
        return;
    };

    task.print_summary();


}

fn main() {
    use std::vec;

let mut my_tasks = vec![
    Tasks {
        title: String::from("Build sender"),
        priority: Priority::High,
        status: Status::InProgress,
        assignee: Some(String::from("Ada")),
    },
    Tasks {
        title: String::from("Build receiver"),
        priority: Priority::Medium,
        status: Status::Todo,
        assignee: None,
    },
    Tasks {
        title: String::from("Fix relay issue"),
        priority: Priority::High,
        status: Status::Blocked(String::from("waiting on vendor API")),
        assignee: Some(String::from("Ben")),
    },
    Tasks {
        title: String::from("Write README"),
        priority: Priority::Low,
        status: Status::Done,
        assignee: Some(String::from("Chen")),
    },
];

    show_full_list(&my_tasks);

    show_task_at_index(&my_tasks, 0);
    show_task_at_index(&my_tasks, 1);

    my_tasks[2].start();
    my_tasks[3].start();

    my_tasks[0].stop();
    my_tasks[1].block();

    println!("\nAfter updates:\n");
    show_full_list(&my_tasks);



}
