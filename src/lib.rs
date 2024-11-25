mod todo;

use crate::todo::{
    core::{ManageTodo, Todo},
    manage, utility,
};

pub fn run() -> Result<(), String> {
    let mut todo_list: Vec<Todo> = Vec::new();

    loop {
        println!("\nAvailable Commands");
        println!("[1] Create Todo");
        println!("[2] Read Todo");
        println!("[3] Update Todo");
        println!("[4] Search Todo");
        println!("[5] Delete Todo");
        println!("[6] Exit Program");
        println!("============================\n");

        print!("Input Command: ");
        utility::stdout_flush()?;

        let entry = match utility::input_entry() {
            Err(err) => {
                println!("\n{err}");
                continue;
            }
            Ok(e) => e,
        };

        let validate = match utility::validate_entry(&entry) {
            Err(err) => {
                println!("\n{err}");
                continue;
            }
            Ok(v) => v,
        };

        match validate {
            ManageTodo::CreateTodo => manage::create_todo(&mut todo_list)?,
            ManageTodo::ReadTodo => manage::read_todo_list(&mut todo_list),
            ManageTodo::UpdateTodo => {
                manage::update_todo_list(&mut todo_list).unwrap_or_else(|err| {
                    println!("\n{err}");
                })
            }
            ManageTodo::SearchTodo => manage::search_todo(&mut todo_list).unwrap_or_else(|err| {
                println!("\n{err}");
            }),
            ManageTodo::DeleteTodo => manage::delete_todo(&mut todo_list).unwrap_or_else(|err| {
                println!("\n{err}");
            }),
            ManageTodo::ExitProgram => {
                println!("Exit Program...");
                return Ok(());
            }
        }
    }
}
