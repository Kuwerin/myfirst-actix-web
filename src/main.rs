mod model;
mod config;
mod handler;
mod repository;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use deadpool_postgres::Runtime;
use tokio_postgres::NoTls;

use crate::model::Status;
use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    env_logger::init();

    dotenv::dotenv().ok();

    let config = Config::from_env().unwrap();

    let pool = config.postgres.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    println!("server is running on: http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move||
        App::new()
            // default logger middleware
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/test", web::get().to(handler::index))
            .route("/todos{_:/?}", web::get().to(handler::get_todos))
            .service(handler::get_items)
            .service(handler::create_todo)
    )
        .bind("127.0.0.1:8000")?
        .run()
        .await
}