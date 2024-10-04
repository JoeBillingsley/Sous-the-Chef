use ingredients::store::ingredient_store::IngredientStore;

pub mod ingredients;

fn main() {
    let ingredient_store = IngredientStore::default();

    let ingredients = ingredient_store.load_ingredients();

    println!("{0}", ingredients.len());

    // TODO:
    // [] First try having an algorithm generate some combination of ~40 ingredients of varying portion sizes that meet some calories / carb / etc. target
    // [] Then could look if enabling recipes with optional values is easier
    //      [] Might want to split foods into more categories if I go that way

    for ingredient in ingredients {
        println!("{0:?}", ingredient.name)
    }
}
