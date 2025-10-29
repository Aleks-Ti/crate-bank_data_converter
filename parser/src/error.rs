#[derive(Debug)]
/// Контейнер с ошибками.
pub enum ParseError {
    /// Ошибка данных полученных в Read.
    Io(std::io::Error),
    /// Не корректные данные в структуре.
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

impl std::error::Error for ParseError {}
