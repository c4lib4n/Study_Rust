
// Problem 1:
/* You are tasked with implementing a library management system using Rust.
Your goal is to design a program that can handle books and magazines.
To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input
and displays its information. The function should output
the item's ID, title, publication year, and type (book or magazine).
*/

// Solution:


#[derive(Debug)]
struct Item {
    id: i32,
    title: String,
    year: i32,
    type_: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

fn display_item_info(item: &Item) {
    println!("Item ID: {:?}", item.id);
    println!("Title: {:?}", item.title);
    println!("Publication Year: {:?}", item.year);
    println!("Publication Type: {:?}", item.type_);
}

fn main() {
    let rust_book = Item {
        id: 1,
        title: String::from("The Rust Programming Language Book"),
        year: 2021,
        type_: ItemType::Book,
    };

    let rust_magazine = Item {
        id: 2,
        title: String::from("Rust Magazine"),
        year: 2022,
        type_: ItemType::Magazine,
    };

    display_item_info(&rust_book);
    display_item_info(&rust_magazine);
}
