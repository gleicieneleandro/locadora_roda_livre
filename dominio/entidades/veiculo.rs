use serde::{Deserialize, Serialize};

use crate::dominio::objetos_valor::categoria::Categoria;
use crate::dominio::objetos_valor::placa::Placa;
use crate::dominio::objetos_valor::status::Status;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Veiculo {
    pub placa: Placa,
    pub modelo: String,
    categoria: Categoria,
    pub status: Status,
}

impl Veiculo {
    pub fn novo(placa: Placa, modelo: String, categoria: Categoria) -> Self {
        Self {
            placa,
            modelo,
            categoria,
            status: Status::Disponivel,
        }
    }

    pub fn diaria(&self) -> f64 {
        self.categoria.diaria_base()
    }

    pub fn disponivel(&self) -> bool {
        matches!(self.status, Status::Disponivel)
    }

    pub fn alugar(&mut self, cpf: String) {
        self.status = Status::Alugado(cpf);
    }
    #[allow(dead_code)]
    pub fn devolver(&mut self) {
        self.status = Status::Disponivel;
    }
}
use crate::dominio::entidades::pessoa::Apresentavel;

impl Apresentavel for Veiculo {
    fn apresentar(&self) -> String {
        format!(
            "Veículo: {} | Modelo: {} | Status: {:?}",
            self.placa.valor(),
            self.modelo,
            self.status
        )
    }
}
