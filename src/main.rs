#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use crate::handlers::{create_task, get_all_tasks, get_task, update_task, delete_task};
use crate::models::establish_connection;

mod schema;
mod models;
mod handlers;
mod errors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));    
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/tasks")
                    .route("", web::post().to(create_task))
                    .route("", web::get().to(get_all_tasks))
                    .route("/{id}", web::get().to(get_task))
                    .route("/{id}", web::put().to(update_task))
                    .route("/{id}", web::delete().to(delete_task)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
