use crate::dominio::entidades::locacao::Locacao;

pub trait PoliticaTarifa {
    fn nome(&self) -> &str;
    fn cobrar(&self, locacao: &Locacao) -> f64;
}

pub struct Balcao;

pub struct Mensalista;

impl PoliticaTarifa for Balcao {
    fn nome(&self) -> &str {
        "Balcao"
    }

    fn cobrar(&self, locacao: &Locacao) -> f64 {
        locacao.valor()
    }
}

impl PoliticaTarifa for Mensalista {
    fn nome(&self) -> &str {
        "Mensalista"
    }

    fn cobrar(&self, locacao: &Locacao) -> f64 {
        locacao.valor() * 0.9
    }
}
