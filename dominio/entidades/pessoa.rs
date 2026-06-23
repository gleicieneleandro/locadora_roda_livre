use serde::{Deserialize, Serialize};

use crate::dominio::objetos_valor::cpf::Cpf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pessoa {
    pub nome: String,
    cpf: Cpf,
}

pub trait Apresentavel {
    fn apresentar(&self) -> String;
}

impl Pessoa {
    pub fn nova(nome: String, cpf: Cpf) -> Self {
        Self { nome, cpf }
    }

    pub fn cpf(&self) -> &Cpf {
        &self.cpf
    }
}

impl Apresentavel for Pessoa {
    fn apresentar(&self) -> String {
        format!("{} | CPF: {}", self.nome, self.cpf.valor())
    }
}
