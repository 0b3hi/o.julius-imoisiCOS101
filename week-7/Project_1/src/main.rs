//Rust program to calculate the area and volume of shapes.

fn read(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().parse().expect("Enter a valid input")
}
//formula
fn trapezium_area(h:f64, b1:f64, b2:f64) ->f64 {
    h * (b1 + b2) / 2.0
}
fn rhombus_area(d1:f64, d2:f64) ->f64 {
    d1 * d2 / 2.0
}
fn parallelogram_area(b:f64, a:f64) ->f64 {
    b * a
}
fn cube_area(ls:f64) ->f64 {
    6.0 * (ls * ls)
}
fn cylinder_volume(r:f64, h:f64) ->f64 {
    3.142 * (r * r) * h
}

use std::io;

fn main() {
    println!("");
    println!("Select the shape you want to calculate");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of parallelogram");
    println!("4. Area of cube");
    println!("5.Volume of cube");

    let choice = read("Enter your choice (1-5): ");

    match choice as u32 {
        1 => {
            let h = read("Enter the height");
            let b1 = read("Enter base 1");
            let b2 = read("Enter base 2");
            let area = trapezium_area(h, b1, b2);
            println!("Area of Trapezium = {}",area );
        }
        2 => {
            let d1 = read("Enter diagonal 1");
            let d2 = read("Enter diagonal 2");
            let area = rhombus_area(d1, d2);
            println!("Area of a Rhombus = {}", area);
        }
        3 => {
            let b = read("Enter base");
            let a = read("Enter altitude");
            let area = parallelogram_area(b,a);
            println!("Area of parallelogram = {}",area);
        }
        4 => {
            let ls = read("Enter the length of the side");
            let area = cube_area(ls);
            println!("Area of Cube = {}",area);
        }
        5 => {
            let r = read("Enter the radius");
            let h = read("Enter the height");
            let volume = cylinder_volume(r,h);
            println!("Volume of cylinder = {}",volume); 
        }
        _=> {
            println!("Invalid!");
        }
    }
    
    
}
