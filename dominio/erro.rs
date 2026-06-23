use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErroDominio {
    PlacaDuplicada,
    CpfDuplicado,
    VeiculoNaoEncontrado,
    ClienteNaoEncontrado,
    FuncionarioNaoEncontrado,
    VeiculoIndisponivel,
}

impl fmt::Display for ErroDominio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlacaDuplicada => write!(f, "placa ja cadastrada"),
            Self::CpfDuplicado => write!(f, "CPF ja cadastrado"),
            Self::VeiculoNaoEncontrado => write!(f, "veiculo nao encontrado"),
            Self::ClienteNaoEncontrado => write!(f, "cliente nao encontrado"),
            Self::FuncionarioNaoEncontrado => write!(f, "funcionario nao encontrado"),
            Self::VeiculoIndisponivel => write!(f, "veiculo indisponivel"),
        }
    }
}
