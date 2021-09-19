use super::{models, State};
use juniper::FieldResult;
use lazy_static::lazy_static;

pub struct Query;

#[juniper::graphql_object(Context=State)]
impl Query {
    fn api_version() -> String {
        "1.0".into()
    }

    fn todos(context: &State) -> FieldResult<Vec<models::Todo>> {
        Ok(context.todo_repository.todos()?)
    }

    fn todo(context: &State, idx: i32) -> FieldResult<models::Todo> {
        Ok(context.todo_repository.todo(idx)?)
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context=State)]
impl Mutation {
    fn create_todo(context: &State, new_todo: models::NewTodo) -> FieldResult<models::Todo> {
        Ok(context.todo_repository.create_todo(new_todo)?)
    }

    fn update_todo(context: &State, update_todo: models::UpdateTodo) -> FieldResult<models::Todo> {
        Ok(context.todo_repository.update_todo(update_todo)?)
    }

    fn delete_todo(context: &State, idx: i32) -> FieldResult<bool> {
        Ok(context.todo_repository.delete_todo(idx).map(|_| true)?)
    }
}

type Schema = juniper::RootNode<'static, Query, Mutation, juniper::EmptySubscription<State>>;
lazy_static! {
    pub static ref SCHEMA: Schema = Schema::new(Query, Mutation, juniper::EmptySubscription::new());
}
