
use std::fs;
use std::io::Write;

fn main() {
    let categories = "Nigerian Breweries Plc categories:\n

 # Lager
 - 33 Export 
 - Desperados
 - Goldberg
 - Gulder
 - Heineken
 - Star

 # Stout
 - Legend
 - Turbo King
 - Williams

 # Non-Alcoholic 
 - Maltina
 - Amstel Malta 
 - Malta Gold
 - Fayrouz"; 

 let mut file = std::fs::File::create("high_quality_drinks.txt").expect("create failed");
 file.write_all(categories.as_bytes()).expect("write failed");
 
}
