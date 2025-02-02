use super::ingredient_store::IngredientStore;
use crate::ingredients::model::recipe::Recipe;
use cooklang::Converter;
use cooklang::CooklangParser;
use cooklang::Extensions;
use glob::glob;
use std::fs;
use std::path::Path;

pub struct RecipeStore {}

impl RecipeStore {
    pub fn load_recipes(recipes_folder: &str, ingredient_store: &IngredientStore) -> Vec<Recipe> {
        let recipes_path = Path::new(recipes_folder).join("*.cook");
        let parser = CooklangParser::new(Extensions::empty(), Converter::default());

        glob(&recipes_path.to_str().unwrap())
            .expect("Failed to process glob pattern")
            .map(|file| match file {
                Ok(file_path) => {
                    let content = fs::read_to_string(file_path).unwrap();
                    let recipe = parser.parse(&content).unwrap_output();

                    recipe
                        .ingredients
                        .into_iter()
                        .map(|ingredient| {
                            ingredient_store
                                .find_by_name(&ingredient.name)
                                .unwrap_or_else(|| {
                                    panic!("Ingredient not found '{}'", ingredient.name)
                                })
                        })
                        .collect()
                }
                Err(error) => panic!("Error processing file: {error}"),
            })
            .collect()
    }
}
