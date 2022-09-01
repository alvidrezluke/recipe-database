use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use database::init_db;
use dotenv;

mod recipe;
mod database;
mod new_recipe;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/images", "static/images/").show_files_listing())
            .service(hello)
            .service(echo)
            .service(new_recipe::get_new_recipe)
            .service(new_recipe::post_new_recipe)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

