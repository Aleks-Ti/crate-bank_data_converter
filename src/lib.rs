use std::{fs::File, io::Read};

use clap::ValueEnum;
pub mod parser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Csv,
    Mt940,
    Camt053,
}

struct Converter;

impl Converter {
    pub fn to_csv() {}
    pub fn to_mt940() {}
    pub fn to_camt053(&mut self, file: &File) {}
    pub fn from_csv_to_camt053() {}
    pub fn from_csv_to_mt940() {}
    pub fn from_camt053_to_csv() {}
    pub fn from_camt053_to_mt940() {}
    pub fn from_mt940_to_csv() {}
    pub fn from_mt940_to_camt053() {}
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
