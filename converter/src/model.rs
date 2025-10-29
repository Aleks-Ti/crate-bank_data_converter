/// Базовая структура Транзакции
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// :20: в MT940
    pub reference: String,
    /// :25:
    pub account: String,
    ///  из :61:
    pub amount: f64,
    /// USD, EUR
    pub currency: String,
    /// YYYY-MM-DD
    pub value_date: String,
    /// :86: или комментарий
    pub description: String,
}
