mod model;
mod config;
mod handler;
mod repository;

use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;

use crate::model::Status;
use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::from_env().unwrap();

    let pool = config.postgres.create_pool(None, NoTls).unwrap();

    println!("server is running on: http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move||
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/test", web::get().to(handler::index))
            .route("/todos", web::get().to(handler::get_todos))
    )
        .bind("127.0.0.1:8000")?
        .run()
        .await
}