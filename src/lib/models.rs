use juniper::{GraphQLInputObject, GraphQLObject};
use rusqlite;
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Debug, Deserialize, Serialize, GraphQLObject)]
#[graphql(description = "A thing to be done")]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize, GraphQLInputObject)]
pub struct NewTodo {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, GraphQLInputObject)]
pub struct UpdateTodo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
