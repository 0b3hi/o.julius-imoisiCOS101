//Rust program that takes order from the customer.

use std::io;

fn main() {
    let p:f32=3200.0;
    let f:f32=3000.0;
    let a:f32=2500.0;
    let e:f32=2000.0;
    let w:f32=2500.0;

    println!();
    println!("P = Poundo Yam/Edinkaiko Soup -3,200Naira.");
    println!("F = Fried Rice & Chicken -3,000Naira.");
    println!("A = Amala & Ewedu Soup -2,500Naira.");
    println!("E = Eba & Egusi Soup -2,000Naira");
    println!("W = White Rice & Stew -2,500Naira");
    println!();

    println!("How many food items do you want to get");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Not a valid input");
    let quantity:f32= quantity.trim().parse().expect("Not a valid input");

    println!("Enter the letter of the food you want to order");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid input");
    let choice:String = choice.to_uppercase().trim().parse().expect("Not a valid input");
    
    let mut price:f32 = 0.0;

    if choice == "P" && quantity>0.0{
           price=quantity*p;
        println!("You ordered Poundo Yam/Edinkaiko Soup.");
        println!("Total cost is N{}",price );
    }  else if choice == "F" && quantity>0.0{
              price = quantity * f;
        println!("You ordered Fried Rice & Chicken.");
        println!("Total cost is N{}",price );
    }  else if choice == "A" && quantity>0.0 {
              price = quantity * a;
        println!("You ordered Amala & Ewedu Soup.");
        println!("Total cost is N{}",price );
    }  else if choice == "E" && quantity>0.0 {
              price = quantity * e;
        println!("You ordered Eba & Egusi Soup.");
        println!("Total cost is N{}",price );
    }  else if choice == "W" && quantity>0.0{
               price = quantity * w;
        println!("You ordered White Rice & Stew.");
        println!("Total cost is N{}",price );
    }  else {
        println!("Invalid choice! Please try again.");
    }

    if price >= 10000.0{
    let discount = price * 0.05;
    let new_price = price-discount;
    println!("You have received a discount of {}",discount );
    println!("Your Total is {}",new_price);
    }
    

}
