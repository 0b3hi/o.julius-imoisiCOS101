//Rust program to determine the annual incentive of an employee.

use std::io;

fn main() {
  
  let mut experience = String::new();
  let mut age = String::new();


    println!("Are you experienced in this field? yes or no: ");
    io::stdin().read_line(&mut experience).expect("Not a valid string");
    let experience:&str = experience.trim();

    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:u32 = age.trim().parse().expect("Not a valid number");

     if experience == "yes" {

        if age >= 40{
            println!("The annual incentive of the employee is 1560000 naira");
    }
        else if age >= 30 && age < 40{
            println!("The annual incentive of the employee is 1480000 naira");
    }
        lse{
            println!("The monthly incentive of the employee is 1300000 naira");
    }
     
    };

    if experience == "no"{
        println!("Your annual incentive is 100,000 naira")
    }
  }


