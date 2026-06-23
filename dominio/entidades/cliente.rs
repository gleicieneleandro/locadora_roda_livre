use crate::dominio::entidades::locacao::Locacao;
use crate::dominio::entidades::pessoa::{Apresentavel, Pessoa};
use crate::dominio::objetos_valor::placa::Placa;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cliente {
    pub pessoa: Pessoa,
    pub cnh: String,
    pub locacoes: Vec<Locacao>,
}

impl Cliente {
    pub fn novo(pessoa: Pessoa, cnh: String) -> Self {
        Self {
            pessoa,
            cnh,
            locacoes: Vec::new(),
        }
    }

    pub fn realizar_locacao(&mut self, placa: Placa, dias: u32, preco_diario: f64, data: String) {
        let locacao = Locacao::nova(placa, dias, preco_diario, data);

        self.locacoes.push(locacao);
    }

    pub fn gasto_total(&self) -> f64 {
        self.locacoes.iter().map(|l| l.valor()).sum()
    }
}

impl Apresentavel for Cliente {
    fn apresentar(&self) -> String {
        format!("{} | CNH: {}", self.pessoa.apresentar(), self.cnh)
    }
}
