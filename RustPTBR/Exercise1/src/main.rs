use std::io;

use Exercise1::{convert_to_int};

fn main() {


    let mut medias = String::new();
    println!("Digite a quantidade de notas:");
    io::stdin().read_line(&mut medias).expect("Error ao ler medias");
    let mut soma_rec = 0;
    let mut i =0;
    while convert_to_int(&medias) > i{
        let mut media_aluno = String::new();
        println!("Digite a nota do aluno {}", i);
        io::stdin().read_line(&mut media_aluno).expect("Error ao ler media do aluno");
        i += 1;
        if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6 {
            soma_rec +=1;
        }
    }
    println!("\n {} alunos em recuperacao", soma_rec);
}
