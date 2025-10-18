use std::io::Read;
pub enum ParseError {
    Io(std::io::Error),
    InvalidFormat(String),
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(err) => write!(f, "IO error: {}", err),
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

pub trait Parser {
    fn parse<R: Read>(input: R) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub struct CsvRow {
    pub row: Vec<String>,
}

pub struct CsvParser {
    pub rows: Vec<CsvRow>,
}

impl Parser for CsvParser {
    fn parse<R: Read>(mut input: R) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer).map_err(ParseError::Io)?;

        if buffer.contains("'") {
            return Err(ParseError::InvalidFormat(
                "Парсер не поддерживает ковычки однойные внутри структуры данных.".to_string(),
            ));
        };

        let mut rows = Vec::new();
        for line in buffer.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let row: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
            rows.push(CsvRow { row });
        }
        Ok(CsvParser { rows })
    }
}
