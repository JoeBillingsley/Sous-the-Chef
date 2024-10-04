use core::panic;
use std::fs::File;

use glob::glob;

use crate::ingredients::model::ingredients::Ingredient;

pub struct IngredientStore {
    glob: String,
}

impl IngredientStore {
    pub fn default() -> IngredientStore {
        Self::new("./ingredients", "*.csv")
    }

    pub fn new(ingredients_path: &str, file_extension: &str) -> Self {
        Self {
            glob: ingredients_path.to_owned() + "/" + file_extension,
        }
    }

    pub fn load_ingredients(&self) -> Vec<Ingredient> {
        glob(&self.glob)
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
            .collect()
    }
}
