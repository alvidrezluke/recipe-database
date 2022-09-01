use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct NewRecipeForm {
    title: String,
    description: String,
    ingredients: Option<String>
}

#[get("/new-recipe")]
pub async fn get_new_recipe() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/new-recipe.html"))
}

#[post("/new-recipe")]
pub async fn post_new_recipe(form_data: web::Json<NewRecipeForm>) -> impl Responder {
    println!("{:?}", form_data);
    HttpResponse::Ok()
}