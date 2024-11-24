use std::io::{self, Write};

use super::core::ManageTodo;

pub fn input_entry() -> Result<String, String> {
    let mut input = String::new();

    let entry = io::stdin().read_line(&mut input);

    match entry {
        Err(e) => return Err(e.to_string()),
        Ok(_) => Ok(input.trim().to_string()),
    }
}

pub fn stdout_flush() -> Result<(), String> {
    match io::stdout().flush() {
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}

pub fn validate_entry(input: &str) -> Result<ManageTodo, String> {
    if input.is_empty() {
        return Err("[Entry cannot be empty]".to_string());
    }

    match input {
        "1" => Ok(ManageTodo::CreateTodo),
        "2" => Ok(ManageTodo::ReadTodo),
        "3" => Ok(ManageTodo::UpdateTodo),
        "4" => Ok(ManageTodo::SearchTodo),
        "5" => Ok(ManageTodo::DeleteTodo),
        "6" => Ok(ManageTodo::ExitProgram),
        _ => Err("[Invalid entry]".to_string()),
    }
}
