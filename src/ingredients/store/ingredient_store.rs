use crate::ingredients::model::ingredients::Ingredient;
use core::panic;
use glob::glob;
use std::{fs::File, path::Path};

pub struct IngredientStore {
    ingredients: Vec<Ingredient>,
}

impl IngredientStore {
    pub fn new(ingredients_folder: &str) -> Self {
        let ingredients_path = Path::new(ingredients_folder).join("*.csv");

        let ingredients = glob(&ingredients_path.to_str().unwrap())
            .expect("Failed to process glob pattern")
            .flat_map(|file| match file {
                Ok(file_path) => {
                    let file = File::open(file_path).unwrap();
                    let mut rdr = csv::Reader::from_reader(file);

                    rdr.deserialize()
                        .map(|row| row.expect("Should deserialise csv row"))
                        .collect::<Vec<Ingredient>>()
                }
                Err(error) => panic!("Error processing file: {error}"),
            })
            .collect();

        Self { ingredients }
    }

    pub fn find_by_name(&self, name: &str) -> Option<Ingredient> {
        self.ingredients
            .iter()
            .find(|ingredient| ingredient.name.to_lowercase() == name.to_lowercase())
            .cloned()
    }
}
