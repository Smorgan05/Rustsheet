use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::io::prelude::*;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {

    let mut file = File::open("file.txt").expect("Failed to open file");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    println!("{}", contents);

    // Open the CSV file
    let file_path = "data.csv";
    let mut file = File::open(file_path)?;

    // Create a CSV reader with default options
    let mut csv_reader = ReaderBuilder::new().from_reader(file);

    // Iterate over each record
    for result in csv_reader.records() {
        // Get the record and unwrap it
        let record = result?;

        // Print each field in the record
        for field in record.iter() {
            println!("{}", field);
        }

        // Add your own logic here to process the record
    }

    Ok(())
}
