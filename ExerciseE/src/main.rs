use std::io;
use ExerciseE::{convert_to_int};

fn main() {
    let mut number_x = String::new();
    let mut number_y = String::new();
    println!("Type the first number: ");
    io::stdin().read_line(&mut number_x).expect("Error, please type the first number");
    println!("Type the second number: ");
    io::stdin().read_line(&mut number_y).expect("Error, please type the second number");
    let mut number1 = convert_to_int(&number_x);
    let mut number2 = convert_to_int(&number_y);
    while number2 != 0{
        let temp = number2;
        number2 = number1 % number2;
        number1 = temp;
    }

    println!("The bigger common divider between {}and {}is: {} ", number_x,number_y,number1);

}
