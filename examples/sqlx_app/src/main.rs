use ngyn::prelude::*;
use sqlx::{Connection, PgConnection};

#[derive(AppState)]
struct State {
    conn: PgConnection,
}

#[derive(Param)]
struct HandleParam {
    id: i32,
}

#[handler]
async fn handle_get(param: HandleParam, state: &mut State) -> String {
    match sqlx::query!("SELECT * FROM users WHERE id = $1", param.id)
        .fetch_one(&mut state.conn)
        .await
    {
        Ok(record) => record.name.unwrap(),
        Err(_) => "Not found".to_string(),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut app = HyperApplication::default();
    let conn = PgConnection::connect(&database_url).await.unwrap();

    app.set_state(State { conn });

    app.get("/{id}", async_wrap(handle_get));

    println!("Starting server at http://127.0.0.1:8080");

    let _ = app.listen("0.0.0.0:8080").await;
}
