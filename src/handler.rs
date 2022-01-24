use actix_web::{HttpResponse, Responder, web, get, post};
use deadpool_postgres::Pool;
use crate::{repository, Status};
use crate::model::CreateTodoList;

pub async fn index() -> impl Responder{
    web::HttpResponse::Ok()
        .json(Status{status:"okay".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder{
    let client = db_pool
        .get()
        .await
        .expect("db conn error");

    let result = repository::get_todos(&client)
        .await;

    match result {
        Ok(todos) =>  HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::NotFound().into()
    }
}

#[get("/todos/{id}/items")]
pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client = db_pool
        .get()
        .await
        .expect("db conn error");

    let result = repository::get_items(&client, path.0)
        .await;

    match result {
        Ok(todo_item) => HttpResponse::Ok().json(todo_item),
        Err(_) => HttpResponse::NotFound().into()
    }
}

#[post("/todo/")]
pub async fn create_todo(db_poll: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client = db_poll
        .get()
        .await
        .expect("db conn error");

    let create_result= repository::create_todo(&client, json.title.clone())
        .await;

    match create_result {
        Ok(todo_list) => HttpResponse::Ok().json(todo_list),
        Err(_) => HttpResponse::BadRequest().into()
    }
}
