#[derive(Debug)]
struct Recipe {
    title: String,
    description: String,
    ingredients: Vec<Ingredient>,
    servings: String,
}

#[derive(Debug)]
pub struct Ingredient {
    item: String,
    measurement: String,
    quantity: f32
}