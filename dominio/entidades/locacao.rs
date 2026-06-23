use serde::{Deserialize, Serialize};

use crate::dominio::objetos_valor::placa::Placa;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Locacao {
    pub placa: Placa,
    pub dias: u32,
    pub preco_diario: f64,
    pub data: String,
}

impl Locacao {
    pub fn nova(placa: Placa, dias: u32, preco_diario: f64, data: String) -> Self {
        Self {
            placa,
            dias,
            preco_diario,
            data,
        }
    }

    pub fn valor(&self) -> f64 {
        self.dias as f64 * self.preco_diario
    }
}
