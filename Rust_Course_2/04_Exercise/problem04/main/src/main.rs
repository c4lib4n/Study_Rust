// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/

fn main() {
    let input = String::from("1111");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );
}

fn palindrome(input: String) -> bool {
    /* Your Code here */
    let mut is_palindrome = true;
    if input.len() == 0 {
        is_palindrome = true;
    }else {
        let mut last = input.len() -1;
        let mut first = 0;

        let my_vec = input.as_bytes();

        while first < last{
            if my_vec[last] != my_vec[first]{
                is_palindrome = false;
                break;
            }
            last -= 1;
            first += 1;

        }
    }

    is_palindrome
}
