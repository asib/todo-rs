use super::models;
use anyhow::Result;
use dotenv::dotenv;
use rusqlite::Connection;
use std::env;
use std::sync::{Arc, Mutex, MutexGuard};

pub enum DatabaseError {
    SqliteError(rusqlite::Error),
}

pub fn get_connection() -> Result<Connection> {
    dotenv().ok();
    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");

    Ok(Connection::open(database_path)?)
}

#[derive(Clone)]
pub struct DatabaseTodoRepository {
    pub connection: Arc<Mutex<rusqlite::Connection>>,
}

impl DatabaseTodoRepository {
    pub fn new(database_connection: rusqlite::Connection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(database_connection)),
        }
    }

    fn get_conn(&self) -> MutexGuard<rusqlite::Connection> {
        self.connection.lock().unwrap()
    }
}

impl super::domain::TodoRepository for DatabaseTodoRepository {
    fn todos(&self) -> Result<Vec<models::Todo>> {
        let conn = self.get_conn();
        let mut stmt = conn.prepare("SELECT id, title, completed FROM todos")?;
        let result = stmt
            .query_map([], models::Todo::from_row)?
            .filter_map(|v| v.ok())
            .collect::<Vec<models::Todo>>();
        Ok(result)
    }

    fn todo(&self, idx: i32) -> Result<models::Todo> {
        Ok(self.get_conn().query_row_and_then(
            "SELECT id, title, completed FROM todos WHERE id = ?",
            [idx],
            models::Todo::from_row,
        )?)
    }

    fn create_todo(&self, todo: models::NewTodo) -> Result<models::Todo> {
        let conn = self.get_conn();
        conn.execute(
            "INSERT INTO todos (title, completed) VALUES (?, FALSE)",
            [todo.title],
        )?;
        let id = conn.last_insert_rowid();
        Ok(conn.query_row_and_then(
            "SELECT id, title, completed FROM todos WHERE id = ?",
            [id],
            models::Todo::from_row,
        )?)
    }

    fn update_todo(&self, todo_update: models::UpdateTodo) -> Result<models::Todo> {
        self.get_conn().execute(
            "UPDATE todos SET title = ?, completed = ? WHERE id = ?",
            rusqlite::params![todo_update.title, todo_update.completed, todo_update.id],
        )?;
        self.todo(todo_update.id)
    }

    fn delete_todo(&self, idx: i32) -> Result<()> {
        Ok(self
            .get_conn()
            .execute("DELETE FROM todos WHERE id = ?", [idx])
            .map(|_| ())?)
    }
}
