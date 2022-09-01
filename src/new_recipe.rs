use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest, error::ResponseError};
use actix_web::web::Json;
use serde::{Serialize, Deserialize};
use sqlx::Executor;

use crate::database;
use crate::recipe;

#[derive(Deserialize, Debug)]
struct NewRecipeForm {
    title: String,
    servings: String,
    description: String,
    ingredients: Vec<String>,
}

#[get("/new-recipe")]
pub async fn get_new_recipe() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/new-recipe.html"))
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    code: u16,
    message: &'static str,
    error: &'static str
}

#[post("/new-recipe")]
pub async fn post_new_recipe(recipe_data: web::Json<NewRecipeForm>) -> impl Responder {

    if recipe_data.title.trim().is_empty() {
        return get_empty_error("Invalid title. Title can not be empty.");
    }
    if recipe_data.servings.trim().is_empty() {
        return get_empty_error("Invalid servings. Servings can not be empty.");
    }
    if recipe_data.description.trim().is_empty() {
        return get_empty_error("Invalid description. Description can not be empty.");
    }
    if recipe_data.ingredients.is_empty() || recipe_data.ingredients[0].is_empty(){
        return get_empty_error("Invalid ingredients. Ingredients can not be empty.");
    }

    upload_recipe(recipe_data).await.expect("Failed");

    HttpResponse::Ok().body("")
}

fn get_empty_error(message: &'static str) -> HttpResponse {
    let err = ErrorResponse {
        code: 400,
        message,
        error: "BAD_REQUEST"
    };
    return HttpResponse::BadRequest()
        .json(err);
}

async fn upload_recipe(recipe_data: Json<NewRecipeForm>) -> Result<(), &'static str> {

    let mut query: String = "".to_string();

    let rec_key = format!("INSERT INTO recipes (recipe_name, recipe_description, recipe_servings) VALUES ('{}', '{}', '{}') RETURNING recipe_id", recipe_data.title.replace('"', ""), recipe_data.description.replace('"', ""), recipe_data.servings.replace('"', ""));

    for ingredient in &recipe_data.ingredients {

        // TODO: Check for ingredient data that is identical and just provide that id to ing_query

        // Split ingredient into parts for database
        let mut split_ingredient  = ingredient.split_whitespace();
        let qty = split_ingredient.next().expect("Invalid ingredient");
        let unt = split_ingredient.next().expect("Invalid ingredient");
        let ing = split_ingredient.collect::<String>();

        // Create all keys for query
        let ing_key = format!("INSERT INTO ingredients (ingredient_name) VALUES ('{}') RETURNING ingredient_id", "flour");
        let qty_key = format!("INSERT INTO measurement_qty (qty_amount) VALUES ('{}') RETURNING measurement_qty_id", "1.5");
        let unt_key = format!("INSERT INTO measurement_units (measurement_description) VALUES ('{}') RETURNING measurement_id", "cups");

        // Create full query for this ingredient
        let ing_query = format!("WITH rec_key AS ({}), ing_key AS ({}), qty_key AS ({}), unt_key AS ({}) INSERT INTO recipe_ingredients (recipe_id, ingredient_id, measurement_qty_id, measurement_id) SELECT rec_key.recipe_id, ing_key.ingredient_id, qty_key.measurement_qty_id, unt_key.measurement_id FROM rec_key, ing_key, qty_key, unt_key;", rec_key, ing_key, qty_key, unt_key);

        // Add it to the existing query
        query += &*ing_query;
    }

    println!("Query: {}", query);

    let db = database::init_db().await.expect("Could not connect to database.");

    db.execute("").await.expect("TODO: panic message");

    Ok(())
}