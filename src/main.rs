use std::env; 
use std::error::Error;
use csv::Reader;

struct CsvRows {
    header: Vec<String>,        
    record_rows: Vec<Vec<String>>,
}

impl CsvRows {
    fn number_of_rows(&self) -> usize{
        self.record_rows.len()
    }

    fn number_of_columns(&self) -> usize{
        self.header.len()
    }

    fn is_csv_empty(&self) {
        if self.header.is_empty() && self.record_rows.is_empty(){
            println!("The Csv file is empty");
        } else {
            println!("The csv file is not empty");
        }
    }

    fn get_rows(&self, index: usize) -> Result<&Vec<String>, String>{
       self.record_rows
       .get(index)
       .ok_or("Row wasnt found in file".to_string()) 
    }

    fn get_cell(&self, row_index: usize, column_index: usize) -> Option<&String> {
        self.record_rows
            .get(row_index)
            .and_then(|row| row.get(column_index))
    }
}

fn parse_args() -> Result<CsvRows, Box<dyn Error>>{
   let args: Vec<String> = env::args().collect();


   if args.len() < 2 {
       return Err(format!("No arguments (filepaths/name) were provided").into());
   }
   
   let first_arg = &args[1];
    
   let mut reader = Reader::from_path(first_arg)?;

   let header = reader.headers()?;

   let header: Vec<String> = header
        .iter()
        .map(|s| s.to_string())
        .collect();

   let mut record_rows: Vec<Vec<String>> = Vec::new();

   for row in reader.records() {
        let records = row?;
        
        let rows: Vec<String> = records
            .iter()
            .map(|v| v.to_string())
            .collect();
        record_rows.push(rows);
   }
   
   Ok(CsvRows {
        header,
        record_rows,
      }
   )
}


fn main() {
    let csv_reader = match parse_args() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let row = &csv_reader.number_of_rows();

    let columns = &csv_reader.number_of_columns();
    
    let empty = &csv_reader.is_csv_empty();

    let header = &csv_reader.get_rows(4);

    let cell = &csv_reader.get_cell(2, 3);

    println!("ROWS: {:?},\n Columns: {:?},\n {:?}, {:?}", row, columns, header, cell);
}
