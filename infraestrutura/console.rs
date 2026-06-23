use std::io::{self, Write};

pub fn ler(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro de leitura");

    entrada.trim().to_string()
}

pub fn mostrar(msg: &str) {
    println!("{}", msg);
}
