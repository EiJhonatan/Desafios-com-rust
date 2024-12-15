use std::io;

pub fn executar() {
    println!("Desafio 2: Verificador de Palíndromo");
    let mut palavra = String::new();
    println!("Digite a palavra:");
    std::io::stdin()
    .read_line(&mut palavra)
    .expect("erro ao ler a palavra");

    palavra = palavra.trim().to_string();

    let palavra_reversa: String = palavra.chars().rev().collect();

    if palavra == palavra_reversa {
        println!("Sim, é um palíndromo")
    }else {
        println!("Não, não é um palíndromo")
    }
}
