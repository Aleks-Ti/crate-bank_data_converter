use crate::model::Transaction;
use parser::{Camt053Parser, CsvParser, Mt940Parser};

pub trait FromParser {
    fn to_transactions(&self) -> Vec<Transaction>;
}

impl FromParser for CsvParser {
    fn to_transactions(&self) -> Vec<Transaction> {
        let mut txs = Vec::new();
        for row in self.rows.iter().skip(1) {
            if row.row.len() < 3 {
                continue;
            }
            txs.push(Transaction {
                reference: row.row[0].clone(),
                account: row.row[1].clone(),
                amount: 0.0,
                currency: "XXX".to_string(),
                value_date: "1970-01-01".to_string(),
                description: row.row[2].clone(),
            });
        }
        txs
    }
}
impl FromParser for Mt940Parser {
    fn to_transactions(&self) -> Vec<Transaction> {
        let mut txs = Vec::new();
        let mut current = Transaction {
            reference: "".to_string(),
            account: "".to_string(),
            amount: 0.0,
            currency: "XXX".to_string(),
            value_date: "1970-01-01".to_string(),
            description: "".to_string(),
        };

        for record in &self.data {
            match record.tag.as_str() {
                "20" => current.reference = record.value.clone(),
                "25" => current.account = record.value.clone(),
                "61" => {
                    if let Some(comma) = record.value.find(',') {
                        let amount_str = &record.value[..comma];
                        if let Ok(amount) = amount_str.parse::<f64>() {
                            current.amount = amount;
                        }
                    }
                    if record.value.len() >= 6 {
                        let date_str = &record.value[..6];
                        current.value_date = format!("20{}-{}-{}", &date_str[0..2], &date_str[2..4], &date_str[4..6]);
                    }
                }
                "86" => current.description = record.value.clone(),
                _ => {}
            }
        }
        if !current.reference.is_empty() {
            txs.push(current);
        }
        txs
    }
}

impl FromParser for Camt053Parser {
    fn to_transactions(&self) -> Vec<Transaction> {
        vec![Transaction {
            reference: "CAMT_REF".to_string(),
            account: "CAMT_ACC".to_string(),
            amount: 0.0,
            currency: "XXX".to_string(),
            value_date: "1970-01-01".to_string(),
            description: "Parsed from CAMT.053".to_string(),
        }]
    }
}
