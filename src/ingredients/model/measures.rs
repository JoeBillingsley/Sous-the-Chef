#[derive(Debug, serde::Deserialize)]
pub enum Measure {
    Ml(usize),
    Gram(usize),
    Unit(usize),
}

pub enum SpoonSizes {
    Tsp,
    Tbsp,
}
