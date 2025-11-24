fn main() {

    let names = vec![
        "Aigbogun A lamba Daudu".to_string(),
        "Murtala Afeez Bendu".to_string(),
        "Okorocha Calistus Ogbona".to_string(),
        "Adewale Jimoh Akanbi".to_string(),
        "Osazuwa Faith Etieye".to_string(),
    ];

    let ministries = vec![
       "Internal Affairs".to_string(),
       "Justice".to_string(),
       "Defense".to_string(),
       "Power & Steel".to_string(),
       "Petroleum".to_string(),
    ];

    let zones = vec![
        "South West".to_string(),
        "North East".to_string(),
        "South South".to_string(),
        "South West".to_string(),
        "South East".to_string(),
        ];

    println!("Merged Commissioner Files");

    for i in 0..names.len() {
        println!(
            "{}. Name: {}, Ministry: {}, Zone: {}",
            i + 1,
            names[i],
            ministries[i],
            zones[i],

             );
    }
}
