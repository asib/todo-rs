use juniper::http::{graphiql, GraphQLRequest};
use tide::http::mime;
use tide::{Request, StatusCode};

pub mod lib;
use lib::{database, graphql, State};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let connection = database::get_connection()?;
    let mut app = tide::with_state(State::new(database::DatabaseTodoRepository::new(
        connection,
    )));
    app.at("/graphql").post(graphql_handler);
    app.at("/graphiql").get(graphiql_handler);
    app.listen("127.0.0.1:8000").await?;

    Ok(())
}

async fn graphql_handler<'a>(mut req: Request<State>) -> tide::Result {
    let query: GraphQLRequest = req.body_json().await?;
    let response = query.execute(&graphql::SCHEMA, req.state()).await;
    let status = if response.is_ok() {
        StatusCode::Ok
    } else {
        StatusCode::BadRequest
    };

    Ok(tide::Response::builder(status)
        .body(tide::Body::from_json(&response)?)
        .build())
}

async fn graphiql_handler<'a>(_: Request<State>) -> tide::Result<impl Into<tide::Response>> {
    Ok(tide::Response::builder(200)
        .body(graphiql::graphiql_source("/graphql", None))
        .content_type(mime::HTML))
}
