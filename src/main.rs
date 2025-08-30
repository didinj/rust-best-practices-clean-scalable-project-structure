mod utils;
mod services;
mod models;
mod errors;

use crate::utils::util::add;
use rust_project_structure_demo::run_app;
use services::greet::greet;
use crate::errors::MyAppError;

fn main() {
    println!("Hello, Rust project structure!");
    greet("Alice");

    let result = add(2, 3);
    println!("Result: {}", result);

    let user = models::user::User {
        id: 1,
        name: String::from("Alice"),
    };

    println!("User: {} with ID {}", user.name, user.id);

    run_app();
}

pub fn process_input(input: &str) -> Result<(), MyAppError> {
    if input.is_empty() {
        return Err(MyAppError::InvalidInput("Input cannot be empty".into()));
    }
    Ok(())
}
