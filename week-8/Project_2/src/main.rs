use std::io;

fn main() {
    let mut names: Vec<String> = Vec::new();
    let mut years: Vec<u32> = Vec::new();

    println!("How many developers?");
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let count: usize = num.trim().parse().unwrap();

    for _i in 0..count {
        let mut name = String::new();
        let mut exp = String::new();

        println!("Enter developer name:");
        io::stdin().read_line(&mut name).unwrap();

        println!("Enter years of experience:");
        io::stdin().read_line(&mut exp).unwrap();

        names.push(name.trim().to_string());
        years.push(exp.trim().parse().unwrap());
    }

    // Assume the first person has the highest experience
    let mut highest_index = 0;

    for i in 0..years.len() {
        if years[i] > years[highest_index] {
            highest_index = i;
        }
    }

    println!(
        "The most experienced developer is {} with {} years.",
        names[highest_index],
        years[highest_index]
    );
}

