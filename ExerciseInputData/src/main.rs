use std::io;

use ExerciseInputData::{convert_to_int};

fn main() {
    let mut number1 = String::new();
    println!("Type the first number: ");
    io::stdin().read_line(&mut number1).expect("Error to read number 1");
    let mut number2 = String::new();
    println!("Type the second number: ");
    io::stdin().read_line(&mut number2).expect("Error to read number 2");

    if convert_to_int(&number1) > convert_to_int(&number2){
        println!("Number {} is great then {}", number1, number2);
    }else {
        println!("Number {} is lower or equal to {}", number1, number2);
    }


}