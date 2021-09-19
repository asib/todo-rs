pub mod database;
pub mod domain;
pub mod graphql;
pub mod models;

#[derive(Clone)]
pub struct State {
    pub todo_repository: Box<dyn domain::TodoRepository>,
}

impl State {
    pub fn new(database_todo_repository: database::DatabaseTodoRepository) -> Self {
        Self {
            todo_repository: Box::new(database_todo_repository),
        }
    }
}

impl juniper::Context for State {}
