use todo_list::run;

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
    })
}
