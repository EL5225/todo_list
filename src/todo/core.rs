pub enum ManageTodo {
    CreateTodo,
    ReadTodo,
    UpdateTodo,
    DeleteTodo,
    SearchTodo,
    ExitProgram,
}

#[derive(Debug)]
pub struct Todo {
    id: u32,
    title: String,
    description: String,
}

impl Todo {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn set_id(&mut self, new_id: u32) {
        self.id = new_id
    }

    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description
    }

    pub fn print(&self) {
        println!(
            "Id: {}\nTitle: {}\nDescription: {}",
            self.id, self.title, self.description
        );
    }
}

impl Drop for Todo {
    fn drop(&mut self) {
        println!("Cleaning {:?}", self);
    }
}
