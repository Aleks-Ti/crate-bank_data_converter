use std::io::Write;

use crate::model::Transaction;

/// Базовый trait для конвертеров форматов.
pub trait ToFormat {
    /// Обязательный метод для всех кто реализует ToFormat.
    ///
    /// Записывает в Write данные преобразования.
    fn from_transactions<W: Write>(txs: &[Transaction], writer: W) -> std::io::Result<()>;
}

fn escape_csv_field(field: &str) -> String {
    if field.contains([',', '"', '\n', '\r']) {
        let escaped = field.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        field.to_string()
    }
}

pub struct CsvFormat;
impl ToFormat for CsvFormat {
    fn from_transactions<W: Write>(txs: &[Transaction], mut writer: W) -> std::io::Result<()> {
        writeln!(writer, "reference,account,amount,currency,date,description")?;

        for tx in txs {
            writeln!(
                writer,
                "{},{},{},{},{},{}",
                escape_csv_field(&tx.reference),
                escape_csv_field(&tx.account),
                tx.amount,
                escape_csv_field(&tx.currency),
                escape_csv_field(&tx.value_date),
                escape_csv_field(&tx.description)
            )?;
        }
        Ok(())
    }
}

pub struct Mt940Format;
impl ToFormat for Mt940Format {
    fn from_transactions<W: Write>(txs: &[Transaction], mut writer: W) -> std::io::Result<()> {
        for tx in txs {
            writeln!(writer, ":20:{}", tx.reference)?;
            writeln!(writer, ":25:{}", tx.account)?;

            let (yy, mm, dd) = if tx.value_date.len() >= 10 {
                (&tx.value_date[2..4], &tx.value_date[5..7], &tx.value_date[8..10])
            } else {
                ("00", "01", "01")
            };

            let sign = if tx.amount < 0.0 { "D" } else { "C" };
            writeln!(writer, ":61:{}{}{}{}{:.2}NMSCNONREF", yy, mm, dd, sign, tx.amount)?;

            if !tx.description.is_empty() {
                writeln!(writer, ":86:{}", tx.description)?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

pub struct Camt053Format;
impl ToFormat for Camt053Format {
    fn from_transactions<W: Write>(txs: &[Transaction], mut writer: W) -> std::io::Result<()> {
        if txs.is_empty() {
            write!(
                writer,
                r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02"></Document>"#
            )?;
            return Ok(());
        }

        let statement_id = &txs[0].reference;
        let account_id = &txs[0].account;

        writeln!(
            writer,
            r#"
            <Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02">
            <BkToCstmrStmt>
            <Stmt>
            <Id>{}</Id>
            <Acct>
                <Id>{}</Id>
            </Acct>
            "#,
            statement_id, account_id
        )?;

        for tx in txs {
            writeln!(
                writer,
                r#"
                <Ntry>
                    <Amt Ccy="{}">{}</Amt>
                    <RvslInd>{}</RvslInd>
                    <AddtlNtryInf>{}</AddtlNtryInf>
                </Ntry>
                "#,
                tx.currency,
                tx.amount.abs(),
                if tx.amount < 0.0 { "true" } else { "false" },
                tx.description
            )?;
        }

        writeln!(
            writer,
            r#"
            </Stmt>
            </BkToCstmrStmt>
            </Document>
            "#
        )?;

        Ok(())
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            reference: "DEFAULT".to_string(),
            account: "DEFAULT".to_string(),
            amount: 0.0,
            currency: "XXX".to_string(),
            value_date: "1970-01-01".to_string(),
            description: "Default".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Parser;
    #[test]
    fn test_multiple_transactions_to_camt053() {
        let txs = vec![
            Transaction {
                reference: "STMT1".to_string(),
                account: "ACC1".to_string(),
                amount: -100.5,
                currency: "USD".to_string(),
                value_date: "2023-01-01".to_string(),
                description: "Debit".to_string(),
            },
            Transaction {
                reference: "STMT1".to_string(),
                account: "ACC1".to_string(),
                amount: 200.0,
                currency: "USD".to_string(),
                value_date: "2023-01-02".to_string(),
                description: "Credit".to_string(),
            },
        ];
        let mut buffer = Vec::new();
        let _ = Camt053Format::from_transactions(&txs, &mut buffer);
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("<Ntry>"));
        assert_eq!(output.matches("<Ntry>").count(), 2);
        assert!(output.contains("100.5"));
        assert!(output.contains("200"));
    }

    #[test]
    fn test_csv_format_with_special_chars() {
        let txs = vec![Transaction {
            reference: "REF,001".to_string(), // содержит запятую
            account: "ACC\"123".to_string(),  // содержит кавычку
            amount: 100.5,
            currency: "USD".to_string(),
            value_date: "2023-01-01".to_string(),
            description: "Line1\nLine2".to_string(), // содержит \n -> должно экранировать
        }];

        let mut buffer = Vec::new();
        CsvFormat::from_transactions(&txs, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains(r#""REF,001""#));
        assert!(output.contains(r#""ACC""123""#));
        assert!(output.contains("\"Line1\nLine2\""));
        println!("{:?}", output);

        let reader = parser::CsvParser::parse(output.as_bytes());
        println!("{:?}", reader);
        assert_eq!(reader.as_ref().unwrap().rows.len(), 2);
        assert_eq!(reader.as_ref().unwrap().rows[1].row.len(), 6); // убедится что "\"Line1\nLine2\"" не разделило на новую rows 
        assert_eq!(reader.as_ref().unwrap().rows[1].row[0], "REF,001");
        assert_eq!(reader.as_ref().unwrap().rows[1].row[1], "ACC\"123");
        assert_eq!(reader.as_ref().unwrap().rows[1].row[5], "Line1\nLine2");
    }
}
