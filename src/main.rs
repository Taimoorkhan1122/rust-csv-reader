use std::{error::Error};
use csv;

fn read_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    
    let mut index = 1;
    for record in reader.records() {
        let result = record?;
        let mut msg = String::new();

        for field in result.iter() {
            msg.push_str(&format!("{field}, "));
        }
        println!("msg {index} => {msg}");
        index +=1;
    }
    Ok(())
}
fn main() {
    if let Err(e) = read_csv("./customers.csv") {
        eprintln!("Error: {}", e);
    }
}
