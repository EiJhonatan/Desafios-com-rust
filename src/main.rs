mod desafio1;
mod desafio2;
mod desafio3;

use std::io;

fn main() {
    println!("Selecione o desafio que deseja executar (1 a 3):");
    let mut escolha = String::new();
    io::stdin()
        .read_line(&mut escolha)
        .expect("Erro ao ler a escolha");
    
    let escolha: u32 = escolha.trim().parse().expect("Por favor, digite um número válido");

    match escolha {
        1 => desafio1::executar(),
        2 => desafio2::executar(),
        3 => desafio3::executar(),
        _ => println!("Escolha inválida!"),
    }
}
