use actix_web::{
    web,
    App,
    HttpServer,
    middleware::Logger,
    error,
    HttpResponse,
};
use sqlx::{
    Pool,
    Postgres,
    postgres::PgPoolOptions,
};
use std::{
    collections::HashMap,
    env,
};
mod routes;
use routes::services;
use lazy_static::lazy_static;

// ------------------------------------------------------------------------------------------------ 

pub struct AppState {
    db_pool: Pool<Postgres>,
}   
// ------------------------------------------------------------------------------------------------ 

lazy_static! {
    static ref HASHMAP: HashMap<i32, i32> = {
        let mut m = HashMap::new();
        m.insert(1, 100000);
        m.insert(2, 80000);
        m.insert(3, 1000000);
        m.insert(4, 10000000);
        m.insert(5, 500000);
        m
    };
}

// ------------------------------------------------------------------------------------------------ 

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "warn");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").unwrap_or("postgres://admin:123@localhost:5432/postgres".to_string());
    let http_port = env::var("HTTP_PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    let pool_size = env::var("POOL_SIZE").unwrap_or("20".to_string()).parse::<u32>().unwrap();
    let actix_workers = env::var("ACTIX_WORKERS").unwrap_or("1".to_string()).parse::<usize>().unwrap();

	let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
		.max_connections(pool_size)
		.connect(&database_url)
		.await
        .expect("Cant create db pool");

    println!("Clients in cache: {:?}\nServing at port: {}", HASHMAP.keys(), http_port);

    let app_state = web::Data::new(AppState {
        db_pool: pool.clone(),
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(app_state.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                error::InternalError::from_response(err, HttpResponse::UnprocessableEntity().finish()).into()
            }))
            .configure(services::config)
    })
    .workers(actix_workers)
    .bind(("0.0.0.0", http_port))
    .expect("Cant bind to address")
    .run()
    .await
}