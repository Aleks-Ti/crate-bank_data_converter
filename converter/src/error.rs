use parser::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("Unsupported conversion: {from} -> {to}")]
    Unsupported { from: String, to: String },
}
