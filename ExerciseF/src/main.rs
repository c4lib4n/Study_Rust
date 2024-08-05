use std::io;
use ExerciseF::{convert_to_int};

fn main() {
    let mut number = String::new();
    println!("Type the number");
    io::stdin().read_line(&mut number).expect("Error when typing the number");
    let mut fat = 1;
    let mut receive_int = convert_to_int(&number);

    while receive_int > 1 {
        fat = fat * receive_int;
        receive_int = receive_int - 1;
    }

    print!("Factorial is {} ", fat);

}
