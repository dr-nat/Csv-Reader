use std::env; 
use std::error::Error;


struct CsvRows {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}


fn parser_args() -> Result<(), Box<dyn Error>>{
   let args: Vec<String> = env::args().collect();

   let _first_arg = &args[1];

   if args.len() < 2 {
       return Err(format!("Error:No arguments were provided").into());
   }

   Ok(())
}


fn main() {
    println!("Hello, world!");
}
