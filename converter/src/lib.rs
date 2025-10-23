pub mod error;
mod from_parser;
mod model;
mod to_format;

pub use error::ConvertError;
pub use from_parser::FromParser;
pub use model::Transaction;
pub use to_format::ToFormat;

use parser::{Camt053Parser, CsvParser, Mt940Parser, Parser};
use std::io::Read;

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

pub fn convert<R: Read>(input: R, from: &Format, to: &Format) -> Result<String, ConvertError> {
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

    let output = match to {
        Format::Csv => to_format::CsvFormat::from_transactions(&transactions),
        Format::Mt940 => to_format::Mt940Format::from_transactions(&transactions),
        Format::Camt053 => to_format::Camt053Format::from_transactions(&transactions),
    };

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_to_mt940() {
        let csv = "ref,acc,comment\nREF123,ACC456,Buy bread";
        let result = convert(csv.as_bytes(), &Format::Csv, &Format::Mt940).unwrap();
        assert!(result.contains(":20:REF123"));
        assert!(result.contains(":25:ACC456"));
    }

    #[test]
    fn test_mt940_to_csv() {
        let mt940 = ":20:REF999\n:25:ACC789\n:61:2301010101DR100,50NMSCNONREF\n:86:Salary\n";
        let result = convert(mt940.as_bytes(), &Format::Mt940, &Format::Csv).unwrap();
        assert!(result.contains("REF999"));
        assert!(result.contains("ACC789"));
    }
}
