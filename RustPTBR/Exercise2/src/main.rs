use std::io;
use Exercise2::{convert_to_int};


fn main(){


    let mut entrada_fatorial = String::new();
    io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada_fatorial");
    let mut fatorial = 1;
    let mut entrada_int = convert_to_int(&entrada_fatorial);

    while entrada_int > 1{
        fatorial = fatorial * entrada_int;
        entrada_int = entrada_int -1;
    }
    println!("Fatorial {} ", fatorial);
}
