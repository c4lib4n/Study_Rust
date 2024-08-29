fn maior_numero(numeros: &[i32]) -> i32{
    let mut maior = numeros[0];
    for num in numeros{
        if num > &maior{
            maior = *num;
        }
    }

    maior
}


fn main() {
    let numeros = [1, 5, 8, 15,55,6,4];
    let maior = maior_numero(&numeros);
    println!("O maior numero eh: {}", maior);
}
