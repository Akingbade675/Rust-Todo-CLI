# Todo List CLI Documentation

## Overview

This Rust program implements a simple Command Line Interface (CLI) for managing a todo list. Users can add tasks, list all tasks, mark tasks as complete, and delete tasks using various commands.

## Features

1. **Add tasks:** Users can add new tasks to the todo list by providing a task description.
2. **List tasks:** Users can view all the tasks currently present in the todo list.
3. **Mark tasks as complete:** Users can mark tasks as complete by specifying the index of the task.
4. **Delete tasks:** Users can remove tasks from the todo list by specifying the index of the task.
5. **Persistence:** The todo list is persisted to a file (`todo.txt`) so that tasks are not lost when the program exits.

## Usage

To run the program, follow these steps:

1. **Compile the program:** Open a terminal and navigate to the directory containing the Rust source code (`lib.rs`, `task.rs`, `todo.rs`).

    ```
    $ cd src
    $ rustc lib.rs
    ```

2. **Run the compiled binary:** Execute the compiled binary in the terminal.

    ```
    $ ./lib
    ```

3. **Interact with the program:** Use the following commands to manage the todo list:
    - `todo add "<description>"`: Add a new task with the specified description.
    - `todo list`: List all tasks currently in the todo list.
    - `todo complete <index>`: Mark a task at the specified index as complete.
    - `todo delete <index>`: Delete a task at the specified index.
    - `quit`: Exit the program and save the todo list to the file.

## Example Usage

```
__________________________________________________
|             WELCOME TO TODO CLI                 |
|_________________________________________________|
| COMMANDS:-                                      |
| todo add "<description>" - Add a new task      |
| todo list - List all tasks                     |
| todo complete <index> - Mark a task as complete |
| todo delete <index> - Delete a task             |
| quit - Exit the program                         |
|_________________________________________________|

todo list
1. [x] todo 1
2. [x] todo 2
3. [] todo 3
```
