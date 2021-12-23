use std::io;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::model::{TodoList};

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error>{
    let query = client.prepare("SELECT * FROM todo_list")
        .await
        .unwrap();

    let todos = client.query(&query, &[])
        .await
        .expect("error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}