// Problem 1: The elements in the vector must be of the same type. In this exercise, we will look at a way-out for storing elements of different types in a vector.

#[derive(Debug)]
enum Value {
    num1(i32),
    num4(f32),

    // Add code here
    // Define two variants of Integer and Float (with associated int and float types respectively)
}

fn main() {
    let some_val = vec![Value::num1(12), Value::num4(15.5)];

    for i in some_val {
        match i {
            Value::num1(num) => println!("Integer: {} ", num),
            Value::num4(num) => println!("Float: {}", num),
        }
    }
}