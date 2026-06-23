use crate::dominio::locadora::Locadora;
use std::fs;

const ARQUIVO: &str = "dados_locadora.json";

pub fn salvar(locadora: &Locadora) {
    match serde_json::to_string_pretty(locadora) {
        Ok(json) => match fs::write(ARQUIVO, json) {
            Ok(_) => {}
            Err(e) => println!("⚠️ Erro ao salvar arquivo: {}", e),
        },
        Err(e) => println!("⚠️ Erro ao serializar dados: {}", e),
    }
}

pub fn carregar() -> Option<Locadora> {
    match fs::read_to_string(ARQUIVO) {
        Ok(conteudo) => match serde_json::from_str(&conteudo) {
            Ok(locadora) => Some(locadora),
            Err(e) => {
                println!("⚠️ Erro ao carregar dados: {}", e);
                None
            }
        },
        Err(_) => None,
    }
}
