use serde::{Deserialize}

#[derive(Deserialize)]
pub enum ServingSize {
    Ml(usize),
    Gram(usize),
    Unit,
}

pub enum SpoonSizes {
    Tsp,
    Tbsp,
}

#[derive(Deserialize)]
pub struct Ingredient {
    name: &'static str,
    portion_size: ServingSize,
    calories_pp: usize,
    fat_pp: f64,
    carbs_pp: f64,
    fiber_pp: f64,
    protein_pp: f64,
}

impl Ingredient {
    pub fn new(
        name: &'static str,
        portion_size: ServingSize,
        calories_pp: usize,
        // Macros in grams
        fat_pp: f64,
        carbs_pp: f64,
        fiber_pp: f64,
        protein_pp: f64,
    ) -> Self {
        Self {
            name,
            portion_size,
            calories_pp,
            fat_pp,
            carbs_pp,
            fiber_pp,
            protein_pp,
        }
    }
}

struct Ingredients;

// https://www.tesco.com/groceries/en-GB/products/267995121
// + excel spreadsheet

// May want to move this into a config file if we make any changes to the structure

impl Ingredients {
    pub fn all_ingredients<'a>() -> Vec<Ingredient> {
        
    }
}
