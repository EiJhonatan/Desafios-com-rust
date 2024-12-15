use std::io;

fn main() {
    println!("Desafio 3: Contador de Ocorrências");
    let mut frase = String::new();
    println!("digitar uma frase:");
    io::stdin()
    .read_line(&mut frase)
    .expect("erro ao ler a frase");
    let frase = frase.trim().to_string();

    let mut letra = String::new();
    println!("Digite uma letra:");
    io::stdin()
    .read_line(&mut letra)
    .expect("erro ao ler a letra");
    let letra = letra.trim();

    if letra.len() != 1 {
        println!("Por favor, digite apenas um caractere.");
        return;
    }

    let verificar = frase.matches(letra).count();

    //println!("na frase tem {} da letra {}",Verificar,letra)

    if verificar > 0 {
        println!("A letra '{}' aparece {} vez(es) na frase.", letra, verificar);
    } else {
        println!("A letra '{}' não aparece na frase.", letra);
    }
}