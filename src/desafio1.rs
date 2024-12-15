pub fn executar() {
    loop {
        println!("Desafio 1: Calculadora Simples");
        
        let mut num1 = String::new();
        println!("Digite o primeiro número:");
        std::io::stdin()
            .read_line(&mut num1)
            .expect("Erro ao ler o número!");
        let num1: f64 = num1.trim().parse().expect("Por favor, digite um número válido!");

        let mut num2 = String::new();
        println!("Digite o segundo número:");
        std::io::stdin()
            .read_line(&mut num2)
            .expect("Erro ao ler o segundo número!");
        let num2: f64 = num2.trim().parse().expect("Por favor, digite um número válido!");

        let mut operacao = String::new();
        println!("Escolha o operador (+, -, *, ou /):");
        std::io::stdin()
            .read_line(&mut operacao)
            .expect("Erro ao ler o operador!");

        let resultado = match operacao.trim() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Erro: Divisão por zero!");
                    continue;
                }
            }
            _ => {
                println!("Operação desconhecida!");
                continue;
            }
        };

        println!("O resultado é: {}", resultado);

       
        let mut continuar = String::new();
        println!("Deseja fazer outra operação? (s para sim, qualquer outra tecla para sair):");
        std::io::stdin()
            .read_line(&mut continuar)
            .expect("Erro ao ler a resposta!");

        if continuar.trim().to_lowercase() != "s" {
            println!("Saindo do programa. Até logo!");
            break;
        }
    }
}
