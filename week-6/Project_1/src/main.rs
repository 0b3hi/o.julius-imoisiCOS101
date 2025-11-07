//Rust program that takes order from the customer.

use std::io;

fn main() {

    println!();
    println!("P = Poundo Yam/Edinkaiko Soup -3,200Naira.");
    println!("F = Fried Rice & Chicken -3,000Naira.");
    println!("A = Amala & Ewedu Soup -2,500Naira.");
    println!("E = Eba & Egusi Soup -2,000Naira");
    println!("W = White Rice & Stew -2,500Naira");
    println!();

    println!("Enter the letter of the food you want to order");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid input");
    let choice = choice.trim().to_uppercase();

    if choice == "P" {
        println!("You ordered Poundo Yam/Edinkaiko Soup.  Total = 3,200Naira");
    }  else if choice == "F" {
        println!("You ordered Fried Rice & Chicken.  Total = 3,000Naira");
    }  else if choice == "A" {
        println!("You ordered Amala & Ewedu Soup.  Total = 2,500Naira");
    }  else if choice == "E" {
        println!("You ordered Eba & Egusi Soup.  Total = 2,000Naira");
    }  else if choice == "W" {
        println!("You ordered White Rice & Stew.  Total = 2,500Naira");
    }  else {
        println!("Invalid choice! Please try again.");
    }

}
