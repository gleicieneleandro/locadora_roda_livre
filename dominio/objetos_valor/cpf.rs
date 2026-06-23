use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cpf(String);

impl Cpf {
    pub fn novo(valor: String) -> Self {
        Self(valor)
    }

    pub fn valor(&self) -> &str {
        &self.0
    }
}
