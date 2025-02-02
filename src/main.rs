use ingredients::store::{ingredient_store::IngredientStore, recipe_store::RecipeStore};

pub mod ingredients;

fn main() {
    let ingredient_store = IngredientStore::new("config/ingredients");
    let recipes = RecipeStore::load_recipes("config/recipes", &ingredient_store);

    println!("{:?}", recipes);
}
