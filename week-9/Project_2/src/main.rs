
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() -> std::io::Result<()> {
    // Example data (you can add/remove records or build the Vec from user input)
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric: "CSC10382828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekule Gold".to_string(),
            matric: "EEE10202020".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // Print a simple table to the terminal
    println!(
        "{:<22} {:<15} {:<12} {:>5}",
        "Student Name", "Matric Number", "Department", "Level"
    );
    println!("{}", "-".repeat(60));
    for s in &students {
        println!(
            "{:<22} {:<15} {:<12} {:>5}",
            s.name, s.matric, s.department, s.level
        );
    }

    // Save to CSV file
    let file = File::create("students.csv")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Student Name,Matric Number,Department,Level")?;
    for s in &students {
        // Quote fields if needed (simple CSV)
        writeln!(
            writer,
            "\"{}\",\"{}\",\"{}\",{}",
            s.name, s.matric, s.department, s.level
        )?;
    }

    println!("\nSaved {} records to students.csv", students.len());
    Ok(())
}

