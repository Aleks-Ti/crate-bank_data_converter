use std::{fs::File, io::Read};

pub mod parser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[derive(clap::Parser)]
struct Cli {
    #[arg(short, long, value_enum)]
    in_format: Format,
    #[arg(short, long, value_enum)]
    out_format: Format,
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(clap::ValueEnum, Clone)]
enum Format {
    Csv,
    Mt940,
    Camt053,
}


struct Converter;


impl Converter {
    pub fn to_csv() {
        
    }
    pub fn to_mt940() {
        
    }
    pub fn to_camt053(&mut self, file: &File) {
        let data = file.read(file).unwrap();
    }
    pub fn from_csv_to_camt053() {
        
    }
    pub fn from_csv_to_mt940() {
        
    }
    pub fn from_camt053_to_csv() {
        
    }
    pub fn from_camt053_to_mt940() {
        
    }
    pub fn from_mt940_to_csv() {
        
    }
    pub fn from_mt940_to_camt053() {
        
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
