#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub reference: String,   // :20: в MT940
    pub account: String,     // :25:
    pub amount: f64,         // из :61:
    pub currency: String,    // USD, EUR
    pub value_date: String,  // YYYY-MM-DD
    pub description: String, // :86: или комментарий
}
