fn eh_primo(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limite = (num as f64).sqrt() as u32;

    for i in 2..=limite{
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main(){

    let numero = 7;
    let primo = eh_primo(numero);
    println!("{} é primo ? {}", numero, primo);

}