use serde::{Deserialize, Serialize};

use crate::dominio::entidades::pessoa::{Apresentavel, Pessoa};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Funcionario {
    pub pessoa: Pessoa,
    pub cargo: String,
}

impl Funcionario {
    pub fn novo(pessoa: Pessoa, cargo: String) -> Self {
        Self { pessoa, cargo }
    }
}

impl Apresentavel for Funcionario {
    fn apresentar(&self) -> String {
        format!("{} | Cargo: {}", self.pessoa.apresentar(), self.cargo)
    }
}
