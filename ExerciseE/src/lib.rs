pub fn convert_to_int(data_input: &String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("Plural: {}", s);
    } else {
        println!("Singular: {}", s);
    }
}

pub fn change (s: &mut String){
    if !s.ends_with("s"){
        s.push_str("s");
    }
}

pub fn eat (s: String) -> bool {
    if s.starts_with("b") && s.contains("a"){
        true
    }else {
        false
    }
}

pub fn bedazzle(s: &mut String){
    s.push_str("soarkly");
}