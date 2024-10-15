//Problem 1: Modify the code by eliminating all the let statements

struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit(fruit: Fruit) -> Fruit {
    let fruit = Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    };
    fruit
}

fn new_fruit() -> Fruit {
    let fruit = Fruit {
        apples: 10,
        bananas: 5,
    };
    fruit
}

fn print_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}

fn main() {
    print_fruit(increase_fruit(new_fruit()));
}