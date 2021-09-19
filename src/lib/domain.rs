use super::models;
use anyhow::Result;

pub trait TodoRepository: TodoRepositoryClone + Send + Sync {
    fn todos(&self) -> Result<Vec<models::Todo>>;
    fn todo(&self, idx: i32) -> Result<models::Todo>;
    fn create_todo(&self, todo: models::NewTodo) -> Result<models::Todo>;
    fn update_todo(&self, todo_update: models::UpdateTodo) -> Result<models::Todo>;
    fn delete_todo(&self, idx: i32) -> Result<()>;
}

pub trait TodoRepositoryClone {
    fn clone_box(&self) -> Box<dyn TodoRepository>;
}

impl<T> TodoRepositoryClone for T
where
    T: 'static + TodoRepository + Clone,
{
    fn clone_box(&self) -> Box<dyn TodoRepository> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn TodoRepository> {
    fn clone(&self) -> Box<dyn TodoRepository> {
        self.clone_box()
    }
}
