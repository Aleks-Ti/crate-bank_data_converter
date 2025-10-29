pub mod error;
mod from_parser;
mod model;
mod to_format;

pub use error::ConvertError;
pub use from_parser::FromParser;
pub use model::Transaction;
pub use to_format::ToFormat;

use parser::{Camt053Parser, CsvParser, Mt940Parser, Parser};
use std::{io::Read, io::Write};

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Csv,
    Mt940,
    Camt053,
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "csv" => Format::Csv,
            "mt940" => Format::Mt940,
            "camt053" => Format::Camt053,
            _ => Format::Csv,
        }
    }
}

pub fn convert<R: Read, W: Write>(input: R, from: &Format, to: &Format, output: W) -> Result<(), ConvertError> {
    let transactions = match from {
        Format::Csv => {
            let parser = CsvParser::parse(input)?;
            parser.to_transactions()
        }
        Format::Mt940 => {
            let parser = Mt940Parser::parse(input)?;
            parser.to_transactions()
        }
        Format::Camt053 => {
            let parser = Camt053Parser::parse(input)?;
            parser.to_transactions()
        }
    };
    let _ = match to {
        Format::Csv => to_format::CsvFormat::from_transactions(&transactions, output),
        Format::Mt940 => to_format::Mt940Format::from_transactions(&transactions, output),
        Format::Camt053 => to_format::Camt053Format::from_transactions(&transactions, output),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_format() {
        let txs = vec![/* ... */];
        let mut buffer = Vec::new();
        to_format::CsvFormat::from_transactions(&txs, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("reference,account"));
    }

    #[test]
    fn test_csv_to_mt940() {
        let csv = "ref,acc,comment\nREF123,ACC456,Buy bread";
        let mut buffer = Vec::new();

        convert(csv.as_bytes(), &Format::Csv, &Format::Mt940, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains(":20:REF123"));
        assert!(output.contains(":25:ACC456"));
    }

    #[test]
    fn test_mt940_to_csv() {
        let mt940 = ":20:REF999\n:25:ACC789\n:61:2301010101DR100,50NMSCNONREF\n:86:Salary\n";
        let mut buffer = Vec::new();
        convert(mt940.as_bytes(), &Format::Mt940, &Format::Csv, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("REF999"));
        assert!(output.contains("ACC789"));
    }
}
