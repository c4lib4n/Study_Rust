fn count(num: i32){
     for n in 1..(num + 1){
         println!("{}", n)
     }
}

fn count_down(num: i32){
    let mut n = num;
    while n > 0{
        println!("{}", n);
        n -= 1
    }
}

fn main() {
    println!("Counting up to 10");
    count(10);
    println!("\nCounting down from 10");
    count_down(10);
}
