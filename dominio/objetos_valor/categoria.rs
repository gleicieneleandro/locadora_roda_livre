use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Categoria {
    Economico,
    Suv,
    Luxo,
}

impl Categoria {
    pub fn diaria_base(&self) -> f64 {
        match self {
            Categoria::Economico => 120.0,
            Categoria::Suv => 180.0,
            Categoria::Luxo => 300.0,
        }
    }
}
