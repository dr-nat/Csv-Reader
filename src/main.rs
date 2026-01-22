use std::env; 
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use csv::Reader;

struct CsvRows {
    header: Vec<String>,        
    rows: Vec<Vec<String>>,
}


fn parse_args() -> Result<(), Box<dyn Error>>{
   let args: Vec<String> = env::args().collect();


   if args.len() < 2 {
       return Err(format!("No arguments (filepaths/name) were provided").into());
   }
   
   let first_arg = &args[1];
    
   let mut reader = Reader::from_path(first_arg)?;

   let header = reader.headers()?;
   println!("{:?}", header);

   for rows in reader.records() {
        let records = rows?;
        println!("{:?}", records);
   }
   Ok(())
}


fn main() {
    let csv_reader = parse_args();

    match csv_reader {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    } 
}
