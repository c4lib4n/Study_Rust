
fn main() {

    println!("Por Favor, digite uma sequencia de numeros reais: ");

    let mut entrada = String::new();

    std::io::stdin().read_line(&mut entrada).expect("Falha ao entrar com o numero: ");

    let numeros: Vec<f64> = entrada.trim().split_whitespace().map(|x| x.parse::<f64>().expect("Por favor, insira numeros reais")).collect();

    let mut soma: f64 = 0.0;

    for num in &numeros{
        if num % 2.0 == 0.0{
            soma += num;
        }
    }

    print!("\nA soma dos numeros pares eh: {}", soma);

}

