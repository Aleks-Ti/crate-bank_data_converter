use crate::model::Transaction;
use parser::{Camt053Parser, CsvParser, Mt940Parser};
use regex::Regex;
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
        let mut txs: Vec<Transaction> = Vec::new();
        let mut reference = String::new();
        let mut account = String::new();

        let re = Regex::new(r"^[0-9]{6,10}(C|D)R?([0-9,]+)").unwrap();
        for record in &self.data {
            match record.tag.as_str() {
                "20" => reference = record.value.clone(),
                "25" => account = record.value.clone(),
                "61" => {
                    let mut current = Transaction {
                        reference: reference.clone(),
                        account: account.clone(),
                        amount: 0.0,
                        currency: "XXX".to_string(),
                        value_date: "1970-01-01".to_string(),
                        description: "".to_string(),
                    };

                    let value = &record.value;

                    if value.len() >= 6 {
                        let date_part = &value[..6];
                        current.value_date = format!("20{}-{}-{}", &date_part[0..2], &date_part[2..4], &date_part[4..6]);
                    }

                    if let Some(caps) = re.captures(value) {
                        let op_type = &caps[1];
                        let amount_str = &caps[2];
                        let amount_clean = amount_str.replace(',', ".");
                        if let Ok(mut amount) = amount_clean.parse::<f64>() {
                            if op_type == "D" {
                                amount = -amount;
                            }
                            current.amount = amount;
                        }
                    }
                    txs.push(current);
                }
                "86" => {
                    if let Some(last) = txs.last_mut() {
                        last.description = record.value.clone()
                    }
                }
                _ => {}
            }
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

#[cfg(test)]
mod tests {
    use parser::Parser;

    use super::*;
    #[test]
    fn mt940_amount_dr_should_be_negative() {
        // дебет 100,50 → сумма должна быть -100.50
        let mt940 = ":20:REF\n:25:ACC\n:61:2301010101DR100,50NMSCNONREF\n:86:desc\n";
        let p = Mt940Parser::parse(mt940.as_bytes()).unwrap();

        let txs = FromParser::to_transactions(&p);

        assert_eq!(txs.len(), 1, "должна быть ровно одна транзакция");
        // ❗ На твоём текущем коде amount останется 0.0 → тест УПАДЁТ
        assert!(
            (txs[0].amount - (-100.50)).abs() < 1e-6,
            "ожидали -100.50, а получили {}",
            txs[0].amount
        );
    }
    #[test]
    fn mt940_amount_cr_should_be_positive() {
        let mt940 = ":20:REF2\n:25:ACC2\n:61:230101CR999,99NTRFREF123\n";
        let p = Mt940Parser::parse(mt940.as_bytes()).unwrap();
        let txs = FromParser::to_transactions(&p);
        assert_eq!(txs.len(), 1);
        assert!((txs[0].amount - 999.99).abs() < 1e-6);
    }
    #[test]
    fn mt940_multiple_61_should_produce_multiple_transactions() {
        let mt940 = "\
        :20:REF
        :25:ACC
        :61:230101CR10,00NMSCREF1
        :86:first
        :61:230102DR5,00NMSCREF2
        :86:second
    ";
        let p = Mt940Parser::parse(mt940.as_bytes()).unwrap();
        let txs = FromParser::to_transactions(&p);

        // ❗ На текущем коде будет len()==1 → тест УПАДЁТ
        assert_eq!(txs.len(), 2, "каждый :61: должен создавать новую транзакцию");
    }
}
