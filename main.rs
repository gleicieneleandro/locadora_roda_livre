mod aplicacao;
mod dominio;
mod infraestrutura;

use aplicacao::casos_de_uso::{media_movel, media_ponderada};
use dominio::entidades::cliente::Cliente;
use dominio::entidades::funcionario::Funcionario;
use dominio::entidades::pessoa::{Apresentavel, Pessoa};
use dominio::entidades::veiculo::Veiculo;
use dominio::locadora::Locadora;
use dominio::objetos_valor::categoria::Categoria;
use dominio::objetos_valor::cpf::Cpf;
use dominio::objetos_valor::placa::Placa;
use dominio::politica_tarifa::{Balcao, Mensalista, PoliticaTarifa};
use infraestrutura::repositorio;

fn imprimir_disponiveis(locadora: &Locadora, titulo: &str) {
    println!("\n{titulo}");
    let disponiveis = locadora.listar_disponiveis();

    if disponiveis.is_empty() {
        println!("nenhum");
        return;
    }

    for v in disponiveis {
        println!(
            "- {} ({}) - {:?} - R$ {:.2}/dia",
            v.modelo,
            v.placa.valor(),
            v.status,
            v.diaria()
        );
    }
}

fn main() {
    println!("Passo 10 - carregamento inicial do JSON");
    match repositorio::carregar() {
        Some(l) => println!(
            "Estado anterior carregado de dados_locadora.json: {} veiculos, {} clientes.",
            l.veiculos().len(),
            l.clientes().len()
        ),
        None => println!("Nenhum JSON anterior encontrado; iniciando roteiro da prova."),
    }

    println!("\nPasso 1 - criar a locadora");
    let mut locadora = Locadora::nova();
    println!("Locadora {}", locadora.nome);

    println!("\nPasso 2 - cadastrar 3 veiculos");
    let veiculos = [
        Veiculo::novo(
            Placa::novo("ABC-1234".to_string()),
            "HB20".to_string(),
            Categoria::Economico,
        ),
        Veiculo::novo(
            Placa::novo("GHI-9012".to_string()),
            "Onix".to_string(),
            Categoria::Suv,
        ),
        Veiculo::novo(
            Placa::novo("DEF-5678".to_string()),
            "Gol".to_string(),
            Categoria::Luxo,
        ),
    ];

    for veiculo in veiculos {
        let modelo = veiculo.modelo.clone();
        match locadora.cadastrar_veiculo(veiculo) {
            Ok(()) => println!("Veiculo cadastrado: {modelo}"),
            Err(e) => println!("Erro ao cadastrar veiculo {modelo}: {e}"),
        }
    }

    println!("\nPasso 3 - criar 2 clientes e 1 funcionario");
    let cliente1 = Cliente::novo(
        Pessoa::nova(
            "Ana Lima".to_string(),
            Cpf::novo("111.222.333-44".to_string()),
        ),
        "98765432100".to_string(),
    );
    let cliente2 = Cliente::novo(
        Pessoa::nova(
            "Joao Rosa".to_string(),
            Cpf::novo("222.333.444-55".to_string()),
        ),
        "12345678900".to_string(),
    );
    let funcionario = Funcionario::novo(
        Pessoa::nova(
            "Pedro Souza".to_string(),
            Cpf::novo("333.444.555-66".to_string()),
        ),
        "Atendente".to_string(),
    );
    println!("Clientes e funcionario criados por composicao com Pessoa.");

    println!("\nPasso 4 - cadastrar clientes e funcionario");
    let nome_cliente1 = cliente1.pessoa.nome.clone();
    match locadora.cadastrar_cliente(cliente1) {
        Ok(()) => println!("Cliente cadastrado: {nome_cliente1}"),
        Err(e) => println!("Erro ao cadastrar cliente {nome_cliente1}: {e}"),
    }
    let nome_cliente2 = cliente2.pessoa.nome.clone();
    match locadora.cadastrar_cliente(cliente2) {
        Ok(()) => println!("Cliente cadastrado: {nome_cliente2}"),
        Err(e) => println!("Erro ao cadastrar cliente {nome_cliente2}: {e}"),
    }
    let nome_funcionario = funcionario.pessoa.nome.clone();
    match locadora.cadastrar_funcionario(funcionario) {
        Ok(()) => println!("Funcionario cadastrado: {nome_funcionario}"),
        Err(e) => println!("Erro ao cadastrar funcionario {nome_funcionario}: {e}"),
    }

    println!("\nPasso 5 e 6 - listar antes e realizar 3 locacoes");
    imprimir_disponiveis(&locadora, "Veiculos disponiveis antes das locacoes:");

    let locacoes = [
        ("ABC-1234", 2, "2026-05-01"),
        ("GHI-9012", 3, "2026-05-10"),
        ("DEF-5678", 1, "2026-05-20"),
    ];

    for (placa, dias, data) in locacoes {
        match locadora.alugar_veiculo(placa, "111.222.333-44", dias, data.to_string()) {
            Ok(l) => println!(
                "Ana alugou {} por {} dias - R$ {:.2}/dia ({})",
                l.placa.valor(),
                l.dias,
                l.preco_diario,
                l.data
            ),
            Err(e) => println!("Erro ao alugar {placa}: {e}"),
        }
    }

    imprimir_disponiveis(&locadora, "Veiculos disponiveis depois das locacoes:");

    let ana = locadora
        .clientes()
        .iter()
        .find(|c| c.pessoa.cpf().valor() == "111.222.333-44");

    if let Some(ana) = ana {
        println!("\nPasso 7 - total gasto");
        println!("Total gasto por {}: R$ {:.2}", ana.pessoa.nome, ana.gasto_total());

        println!("\nPasso 8 - medias com iteradores");
        let valores: Vec<f64> = ana.locacoes.iter().map(|l| l.valor()).collect();
        println!("Valores das locacoes: {:?}", valores);
        println!("Media movel (janela=2): {:?}", media_movel(&valores, 2));
        println!(
            "Media ponderada (peso maior para a mais recente): R$ {:.2}",
            media_ponderada(&valores)
        );

        println!("\nPasso 9 - cotacao polimorfica com &dyn PoliticaTarifa");
        if let Some(ultima) = ana.locacoes.last() {
            let balcao = Balcao;
            let mensalista = Mensalista;
            let politicas: [&dyn PoliticaTarifa; 2] = [&balcao, &mensalista];

            for politica in politicas {
                println!("{}: R$ {:.2}", politica.nome(), politica.cobrar(ultima));
            }
        }
    }

    println!("\nPasso 10 - salvar JSON e apresentar envolvidos");
    repositorio::salvar(&locadora);
    println!("Estado salvo em dados_locadora.json");

    for cliente in locadora.clientes() {
        println!("{}", cliente.apresentar());
    }
    for funcionario in locadora.funcionarios() {
        println!("{}", funcionario.apresentar());
    }
    for veiculo in locadora.veiculos() {
        println!("{}", veiculo.apresentar());
    }
}
