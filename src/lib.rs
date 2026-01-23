use std::env; 
use std::error::Error;
use csv::Reader;

pub struct CsvRows {
    pub header: Vec<String>,        
    pub record_rows: Vec<Vec<String>>,
}

impl CsvRows {
    pub fn number_of_rows(&self) -> usize{
        self.record_rows.len()
    }

    pub fn number_of_columns(&self) -> usize{
        self.header.len()
    }

    pub fn is_csv_empty(&self) {
        if self.header.is_empty() && self.record_rows.is_empty(){
            println!("The Csv file is empty");
        } else {
            println!("The csv file is not empty");
        }
    }

    pub fn get_rows(&self, index: usize) -> Result<&Vec<String>, String>{
       self.record_rows
       .get(index)
       .ok_or("Row wasnt found in file".to_string()) 
    }

    pub fn get_cell(&self, row_index: usize, column_index: usize) -> Option<&String> {
        self.record_rows
            .get(row_index)
            .and_then(|row| row.get(column_index))
    }
}

pub fn parse_args() -> Result<CsvRows, Box<dyn Error>>{
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

pub fn run() -> Result<(), Box<dyn Error>> {
    
    let csv_reader = parse_args()?;

    let row_count = &csv_reader.number_of_rows();

    let col_count = &csv_reader.number_of_columns();
    
    let _empty = &csv_reader.is_csv_empty();

    let header_row = &csv_reader.get_rows(4)?;

    let specific_cell = &csv_reader.get_cell(1, 2).ok_or("Cell not found")?;

    println!("ROWS: {},", row_count);
    println!("Columns: {},", col_count);
    println!("Header at index 4: {:?}", header_row);
    println!("Value at (1, 2): {}", specific_cell);


    Ok(())
}


