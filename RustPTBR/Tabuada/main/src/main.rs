pub fn convert_to_int(data_input: &String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number = String::new();
    println!("Type the number");
    std::io::stdin().read_line(&mut number).expect("Error when typing the number");
    let receive_int = convert_to_int(&number);

    for i in 1..11{
        let result = i * receive_int;
        println!("Tabuada do numero {} * {} = {}", receive_int, i, result);
    }


}
