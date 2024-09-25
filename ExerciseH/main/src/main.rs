#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");

    v.iter()
        .filter(|&x| x % 2 == 0) // Filter for even numbers
        .map(|&x| x * x) // Square even numbers
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // Spawn a child thread and get the join handle
    let handle = thread::spawn(move || expensive_sum(my_vector));

    // Main thread does its work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    // Retrieve the result from the child thread
    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {}", sum);

    // Use channels for thread communication
    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(0);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100); // Ensure Thread A starts before Thread B

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // Join the child threads
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    // Challenge: Create two child threads with a receiving end each
    let (tx_challenge, rx_challenge) = channel::unbounded();
    let rx_challenge2 = rx_challenge.clone();

    let handle_challenge_1 = thread::spawn(move || {
        for msg in rx_challenge {
            println!("Child thread 1 received: {}", msg);
        }
    });

    let handle_challenge_2 = thread::spawn(move || {
        for msg in rx_challenge2 {
            println!("Child thread 2 received: {}", msg);
        }
    });

    for value in 1..=5 {
        println!("Main thread sending: {}", value);
        tx_challenge.send(value).unwrap();
    }

    drop(tx_challenge); // Close the sending side

    handle_challenge_1.join().unwrap();
    handle_challenge_2.join().unwrap();

    println!("Main thread: Exiting.");
}