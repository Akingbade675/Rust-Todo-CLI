use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
pub struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String, completed: bool) -> Self {
        Task {
            description,
            completed,
        }
    }

    fn display(&self, index: usize) {
        println!("{}. [{}] {}", index + 1, if self.completed { "x" } else { " " }, self.description);
    }
}

#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<Task>,
    file_path: String,
}

impl TodoList {
    fn new(file_path: String) -> Self {
        TodoList {
            tasks: Vec::new(),
            file_path,
        }
    }

    fn load(&mut self) -> io::Result<()> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let mut parts = line.split("::completed");
            let description = parts.next().unwrap().to_string();
            let completed = parts.next().unwrap();
            self.tasks.push(Task::new(description, completed == "true"));
        }
        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.file_path)?;
        for task in &self.tasks {
            writeln!(file, "{}::completed{}", task.description, task.completed)?;
        }
        Ok(())
    }

    fn add(&mut self, description: String) {
        self.tasks.push(Task::new(description, false));
    }

    fn list(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            task.display(index);
        }
    }

    fn complete(&mut self, index: usize) -> io::Result<()> {
        if index < self.tasks.len() {
            self.tasks[index].completed = true;
            self.save()?;
            println!("Task marked as complete: {}", self.tasks[index].description);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Task not found"))
        }
    }

    fn delete(&mut self, index: usize) -> io::Result<()> {
        if index < self.tasks.len() {
            let removed_task = self.tasks.remove(index);
            self.save()?;
            println!("Task deleted: {}", removed_task.description);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Task not found"))
        }
    }
}


fn main() -> io::Result<()> {
    let file_path = String::from("todo.txt");
    let mut todo_list = TodoList::new(file_path.clone());
    todo_list.load()?;

    loop {
        println!("__________________________________________________");
        println!("|             WELCOME TO TODO CLI                 |");
        println!("|_________________________________________________|");
        println!("| COMMANDS:-                                      |");
        println!("| todo add \"<description>\" - Add a new task     |");
        println!("| todo list - List all tasks                      |");
        println!("| todo complete <index> - Mark a task as complete |");
        println!("| todo delete <index> - Delete a task             |");
        println!("| quit - Exit the program                         |");
        println!("|_________________________________________________|");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let args: Vec<&str> = input.split_whitespace().collect();

        let (command, params) : (String, Option<String>) = {
            if args.len() > 2 {
                (format!("{} {}", args[0].trim().to_lowercase(), args[1].trim().to_lowercase()), Some(args[2..].join(" ")))
            } else {
                if args.len() > 1 {
                    (format!("{} {}", args[0].trim().to_lowercase(), args[1].trim().to_lowercase()), None)
                } else {
                    (format!("{}", args[0].trim().to_lowercase()), None)
                }
                
            }
        };

        match command.as_str() {
            "todo add" => {
                if let Some(description) = params {
                    todo_list.add(description.to_string());
                    todo_list.save()?;
                    println!("Task added: {}", description);
                } else {
                    println!("Please provide a task description.");
                }
            }
            "todo list" => {
                todo_list.list();
            }
            "todo complete" => {
                if let Some(index_str) = params {
                    if let Ok(index) = index_str.parse::<usize>() {
                        todo_list.complete(index - 1)?;
                    } else {
                        println!("Invalid task index.");
                    }
                } else {
                    println!("Please provide the index of the task to complete.");
                }
            }
            "todo delete" => {
                if let Some(index_str) = params {
                    if let Ok(index) = index_str.parse::<usize>() {
                        todo_list.delete(index - 1)?;
                    } else {
                        println!("Invalid task index.");
                    }
                } else {
                    println!("Please provide the index of the task to delete.");
                }
            }
            "quit" => {
                todo_list.save()?;
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }

    Ok(())
}

