use crate::todo::{core::Todo, utility};

pub fn create_todo(todo_list: &mut Vec<Todo>) -> Result<(), String> {
    println!("\n============================");
    println!("\n(Create Todo)");

    print!("\nEnter title: ");
    utility::stdout_flush()?;

    let title = utility::input_entry()?;

    print!("Enter description: ");
    utility::stdout_flush()?;

    let description = utility::input_entry()?;

    let id = todo_list.len() as u32 + 1;

    todo_list.push(Todo::new(id, title, description));
    println!("\n[Todo created]");

    Ok(())
}

pub fn read_todo_list(todo_list: &mut Vec<Todo>) {
    if todo_list.is_empty() {
        println!("\n[Todo list is empty]");
    } else {
        println!("\n============================");
        println!("\n(Read Todo List)");
        todo_list.iter().for_each(|todo| {
            println!("\n=========================");
            todo.print();
            println!("=========================");
        });
    }
}

pub fn update_todo_list(todo_list: &mut Vec<Todo>) -> Result<(), String> {
    if todo_list.is_empty() {
        return Err("[Todo list is empty]".to_string());
    }

    println!("\n============================");
    println!("\n(Update Todo List)");

    print!("\nEnter Todo id: ");
    utility::stdout_flush()?;

    let input = utility::input_entry()?;

    let id = match input.parse::<u32>() {
        Err(e) => return Err(e.to_string()),
        Ok(id) => id,
    };

    let todo = match todo_list.iter_mut().find(|x| x.get_id() == id) {
        Some(todo) => todo,
        None => return Err("[Todo not found]".to_string()),
    };

    print!("\nEnter title: ");
    utility::stdout_flush()?;

    let title = utility::input_entry()?;
    todo.set_title(title);

    print!("Enter description: ");
    utility::stdout_flush()?;

    let description = utility::input_entry()?;
    todo.set_description(description);

    println!("\n[Todo updated]");

    Ok(())
}

pub fn search_todo(todo_list: &mut Vec<Todo>) -> Result<(), String> {
    if todo_list.is_empty() {
        return Err("[Todo list is empty]".to_string());
    }

    println!("\n============ ================");
    println!("\n(Search Todo List)");

    print!("\nEnter Todo title: ");
    utility::stdout_flush()?;

    let input = utility::input_entry()?;

    let todo: Vec<&Todo> = todo_list
        .iter()
        .filter(|x| x.get_title().contains(&input))
        .collect();

    if todo.is_empty() {
        return Err("[Todo not found]".to_string());
    }

    println!("\n[Todo found]");
    todo.iter().for_each(|todo| {
        println!("\n=========================");
        todo.print();
        println!("=========================");
    });

    Ok(())
}

pub fn delete_todo(todo_list: &mut Vec<Todo>) -> Result<(), String> {
    if todo_list.is_empty() {
        return Err("[Todo list is empty]".to_string());
    }

    println!("\n============================");
    println!("\n(Delete Todo)");

    print!("\nEnter Todo id: ");
    utility::stdout_flush()?;

    let input = utility::input_entry()?;

    let id = match input.parse::<u32>() {
        Err(e) => return Err(e.to_string()),
        Ok(id) => id,
    };

    todo_list.retain(|x| x.get_id() != id);
    println!("\n[Todo deleted]");

    for i in 0..todo_list.len() {
        todo_list[i].set_id(i as u32 + 1);
    }

    Ok(())
}
