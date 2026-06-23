use crate::dominio::entidades::cliente::Cliente;
use crate::dominio::entidades::funcionario::Funcionario;
use crate::dominio::entidades::locacao::Locacao;
use crate::dominio::entidades::veiculo::Veiculo;
use crate::dominio::erro::ErroDominio;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Locadora {
    pub nome: String,
    clientes: Vec<Cliente>,
    funcionarios: Vec<Funcionario>,
    veiculos: Vec<Veiculo>,
    locacoes: Vec<Locacao>,
}

impl Locadora {
    pub fn nova() -> Self {
        Self {
            nome: "Roda Livre".to_string(),
            clientes: Vec::new(),
            funcionarios: Vec::new(),
            veiculos: Vec::new(),
            locacoes: Vec::new(),
        }
    }

    pub fn cadastrar_cliente(&mut self, cliente: Cliente) -> Result<(), ErroDominio> {
        if self
            .clientes
            .iter()
            .any(|c| c.pessoa.cpf().valor() == cliente.pessoa.cpf().valor())
        {
            return Err(ErroDominio::CpfDuplicado);
        }

        self.clientes.push(cliente);

        Ok(())
    }

    pub fn cadastrar_funcionario(&mut self, funcionario: Funcionario) -> Result<(), ErroDominio> {
        if self
            .funcionarios
            .iter()
            .any(|f| f.pessoa.cpf().valor() == funcionario.pessoa.cpf().valor())
        {
            return Err(ErroDominio::CpfDuplicado);
        }
        self.funcionarios.push(funcionario);
        Ok(())
    }

    pub fn cadastrar_veiculo(&mut self, veiculo: Veiculo) -> Result<(), ErroDominio> {
        if self
            .veiculos
            .iter()
            .any(|v| v.placa.valor() == veiculo.placa.valor())
        {
            return Err(ErroDominio::PlacaDuplicada);
        }

        self.veiculos.push(veiculo);

        Ok(())
    }

    #[allow(dead_code)]
    pub fn registrar_locacao(&mut self, locacao: Locacao) {
        self.locacoes.push(locacao);
    }

    pub fn clientes(&self) -> &[Cliente] {
        &self.clientes
    }

    pub fn funcionarios(&self) -> &[Funcionario] {
        &self.funcionarios
    }

    pub fn veiculos(&self) -> &[Veiculo] {
        &self.veiculos
    }

    #[allow(dead_code)]
    pub fn locacoes(&self) -> &[Locacao] {
        &self.locacoes
    }
    #[allow(dead_code)]
    pub fn faturamento_total(&self) -> f64 {
        self.locacoes.iter().map(|l| l.valor()).sum()
    }

    #[allow(dead_code)]
    pub fn buscar_veiculo(&self, placa: &str) -> Option<&Veiculo> {
        self.veiculos.iter().find(|v| v.placa.valor() == placa)
    }

    pub fn listar_disponiveis(&self) -> Vec<&Veiculo> {
        self.veiculos.iter().filter(|v| v.disponivel()).collect()
    }

    pub fn alugar_veiculo(
        &mut self,
        placa: &str,
        cpf_cliente: &str,
        dias: u32,
        data: String,
    ) -> Result<Locacao, ErroDominio> {
        let cliente_idx = self
            .clientes
            .iter()
            .position(|c| c.pessoa.cpf().valor() == cpf_cliente)
            .ok_or(ErroDominio::ClienteNaoEncontrado)?;

        let veiculo = self
            .veiculos
            .iter_mut()
            .find(|v| v.placa.valor() == placa)
            .ok_or(ErroDominio::VeiculoNaoEncontrado)?;

        if !veiculo.disponivel() {
            return Err(ErroDominio::VeiculoIndisponivel);
        }

        let locacao = Locacao::nova(veiculo.placa.clone(), dias, veiculo.diaria(), data);
        veiculo.alugar(cpf_cliente.to_string());
        self.clientes[cliente_idx].realizar_locacao(
            locacao.placa.clone(),
            locacao.dias,
            locacao.preco_diario,
            locacao.data.clone(),
        );
        self.locacoes.push(locacao.clone());
        Ok(locacao)
    }
}
