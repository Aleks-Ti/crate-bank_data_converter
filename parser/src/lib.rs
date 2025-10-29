use std::io::Read;
mod error;
use csv::ReaderBuilder;
pub use error::ParseError;

pub trait Parser {
    fn parse<R: Read>(input: R) -> Result<Self, ParseError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct CsvRow {
    pub row: Vec<String>,
}

#[derive(Debug)]
pub struct CsvParser {
    pub rows: Vec<CsvRow>,
}
#[derive(Debug)]
pub struct Mt940Record {
    pub tag: String,
    pub value: String,
}
#[derive(Debug)]
pub struct Mt940Parser {
    pub data: Vec<Mt940Record>,
}

#[derive(Debug)]
pub struct Camt053Parser {
    pub data: String,
}

impl Parser for CsvParser {
    fn parse<R: Read>(input: R) -> Result<Self, ParseError> {
        let reader = ReaderBuilder::new().has_headers(false).from_reader(input);

        let mut rows = Vec::new();
        for result in reader.into_records() {
            let record = result.map_err(|e| ParseError::InvalidFormat(e.to_string()))?;
            let row = record.iter().map(|s| s.to_string()).collect();
            rows.push(CsvRow { row });
        }

        Ok(CsvParser { rows })
    }
}

impl Parser for Mt940Parser {
    fn parse<R: Read>(mut input: R) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer).map_err(ParseError::Io)?;
        if buffer.is_empty() {
            return Err(ParseError::InvalidFormat("Invalid Mt90 line. Пустая структура!".to_string()));
        }
        let mut data = Vec::new();
        for line in buffer.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with(':') {
                continue;
            }
            if let Some(colon_position) = line[1..].find(':') {
                let tag = line[1..=colon_position].to_string();
                let value = line[colon_position + 2..].to_string();
                data.push(Mt940Record { tag, value });
            } else {
                return Err(ParseError::InvalidFormat(
                    "Invalid Mt90 line. Нет типичных двоеточий `:` для структуры".to_string(),
                ));
            }
        }
        Ok(Mt940Parser { data })
    }
}

impl Parser for Camt053Parser {
    fn parse<R: Read>(mut input: R) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer).map_err(ParseError::Io)?;

        if !buffer.trim_start().starts_with("<") {
            return Err(ParseError::InvalidFormat(
                "CAMT.053 invalid. В строке нет открывающей скобки <".to_string(),
            ));
        }
        Ok(Camt053Parser { data: buffer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt940_parse() {
        let input = ":20:REF123\n:25:ACC123\n:60F:C230101USD1000,00\n";
        let parser = Mt940Parser::parse(input.as_bytes()).unwrap();
        assert_eq!(parser.data.len(), 3);
        assert_eq!(parser.data[0].tag, "20");
        assert_eq!(parser.data[0].value, "REF123");
    }

    #[test]
    fn test_mt940_parse_invalid_not_mt940() {
        let input = ":невиданная хрень";
        let result = Mt940Parser::parse(input.as_bytes());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::InvalidFormat(_)));
    }

    #[test]
    fn test_mt940_parse_empty() {
        let input = "";
        let result = Mt940Parser::parse(input.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn test_csv_parse() {
        let input = "fullname, transaction_id, comment\nAleks, 21312312, На мягкие французкие булки\n";
        let parser = CsvParser::parse(input.as_bytes()).unwrap();
        assert_eq!(parser.rows.len(), 2);
        assert_eq!(parser.rows[0].row.len(), 3);
        assert_eq!(parser.rows[0].row[0], "fullname".to_string());
        assert_eq!(parser.rows[1].row[0], "Aleks".to_string());
    }

    #[test]
    fn test_camt053_parse() {
        let input = r#"
        <Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02">
        <BkToCstmrStmt>
            <Stmt>
                <Id>STATEMENT123</Id>
            </Stmt>
        </BkToCstmrStmt>
        </Document>"#;
        let parser = Camt053Parser::parse(input.as_bytes()).unwrap();
        assert!(parser.data.contains("<Document"));
        assert!(parser.data.contains("STATEMENT123"));
    }

    #[test]
    fn test_camt053_parse_invalid_not_xml() {
        let input = "невиданная хрень";
        let result = Camt053Parser::parse(input.as_bytes());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::InvalidFormat(_)));
    }

    #[test]
    fn test_camt053_parse_empty() {
        let input = "";
        let result = Camt053Parser::parse(input.as_bytes());
        assert!(result.is_err());
    }
}
