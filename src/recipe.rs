
#[derive(Debug)]
struct Recipe {
    id: String,
    title: String,
    description: String,
    ingredients: Vec<Ingredient>,
    instructions: String,
    servings: u32,
}

#[derive(Debug)]
struct Ingredient {
    item: String,
    quantity: f32
}