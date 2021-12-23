use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use crate::{repository, Status};

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
