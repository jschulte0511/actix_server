use actix_cors::Cors;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod handler;
use handler::link_handler::LinkHandler;

mod models;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

#[get("/links")]
async fn get_linked_list() -> actix_web::Result<impl Responder> {
    // TODO: Handle response
    let resp = LinkHandler::get_links().unwrap();
    log::info!("Found links: {:?}", resp);
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Please specify a path!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::default()
            //.allowed_origin("http://localhost:8080") // Need env variable
            .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .service(hello)
            .service(get_linked_list)
    })
    .bind(("0.0.0.0", 8000))? //Env variable
    .run()
    .await
}
