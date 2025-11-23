
use std::io;

fn main() {
    let aps_level = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL 1 8-10",
        "EL 2 10-13",
        "SES",
    ];

    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "-",
        "Research Assistant",
        "phD Candidiate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom teacher",
        "Snr teacher",
        "Leading teacher",
        "Deputy principal",
        "Principal",
    ];

    println!("Enter profession (OfficeAdmin, Academic, Lawyer, Teacher):");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).unwrap();
    let profession = profession.trim().to_lowercase();

    println!("Enter job title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_string();

    let list = match profession.as_str() {
        "lawyer" => &lawyer,
        "teacher" => &teacher,
        "OfficeAdmin" =>&office_admin,
        "Academic" => &academic,
        _ => {
            println!("Invalid profession!");
            return;
        }
    };

    match list.iter().position(|&x| x.eq_ignore_ascii_case(&title)) {
        Some(i) => println!("APS level: {}", aps_level[i]),
        None => println!("Job not found in this profession."),
     }
  }
  

