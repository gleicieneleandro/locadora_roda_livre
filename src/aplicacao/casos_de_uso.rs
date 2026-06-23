use crate::dominio::entidades::cliente::Cliente;
use crate::dominio::entidades::veiculo::Veiculo;

#[allow(dead_code)]
pub fn calcular_locacao(cliente: &Cliente, veiculo: &Veiculo, dias: u32) -> f64 {
    let _ = cliente;

    veiculo.diaria() * dias as f64
}

pub fn media_movel(valores: &[f64], janela: usize) -> Vec<f64> {
    valores
        .windows(janela)
        .map(|w| w.iter().sum::<f64>() / janela as f64)
        .collect()
}
pub fn media_ponderada(valores: &[f64]) -> f64 {
    if valores.is_empty() {
        return 0.0;
    }
    let n = valores.len();
    let soma_produtos: f64 = valores
        .iter()
        .enumerate()
        .map(|(i, v)| v * (i + 1) as f64)
        .sum();
    let soma_pesos: f64 = (1..=n).map(|i| i as f64).sum();
    soma_produtos / soma_pesos
}
