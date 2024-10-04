use ingredients::store::ingredient_store::IngredientStore;

pub mod ingredients;

fn main() {
    let ingredient_store = IngredientStore::default();

    let ingredients = ingredient_store.load_ingredients();

    println!("{0}", ingredients.len());

    for ingredient in ingredients {
        println!("{0:?}", ingredient.pack_size)
    }
}
