use super::measures::Measure;
use serde::{Deserialize, Deserializer}; // 1.0.94

#[derive(Clone, Debug, Deserialize)]
pub struct Ingredient {
    pub name: String,
    #[serde(deserialize_with = "from_enum")]
    pub pack_size: Measure,
    #[serde(deserialize_with = "from_enum")]
    pub portion_size: Measure,
    pub calories_pp: usize,
    pub fat_pp: f64,
    pub carbs_pp: f64,
    pub fiber_pp: f64,
    pub protein_pp: f64,
}

fn from_enum<'de, D>(deserializer: D) -> Result<Measure, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    let (unit_str, remainder) = s.split_once('(').unwrap();
    let (value_str, _) = remainder.split_once(')').unwrap();

    let value: usize = value_str.parse().unwrap();

    let parsed_unit = match unit_str {
        "Gram" => Measure::Gram(value),
        "Unit" => Measure::Unit(value),
        _ => panic!("")
    };

    Ok(parsed_unit)
}
