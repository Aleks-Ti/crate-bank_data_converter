use std::io::Write;

use crate::model::Transaction;

pub trait ToFormat {
    fn from_transactions<W: Write>(txs: &[Transaction], writer: W) -> std::io::Result<()>;
}

pub struct CsvFormat;
impl ToFormat for CsvFormat {
    fn from_transactions<W: Write>(txs: &[Transaction], mut writer: W) -> std::io::Result<()> {
        writeln!(writer, "reference,account,amount,currency,date,description")?;
        for tx in txs {
            writeln!(
                writer,
                "{},{},{},{},{},{}",
                tx.reference, tx.account, tx.amount, tx.currency, tx.value_date, tx.description
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
                (
                    &tx.value_date[2..4],
                    &tx.value_date[5..7],
                    &tx.value_date[8..10],
                )
            } else {
                ("00", "01", "01")
            };

            let sign = if tx.amount < 0.0 { "D" } else { "C" };
            writeln!(
                writer,
                ":61:{}{}{}{}{:.2}NMSCNONREF",
                yy, mm, dd, sign, tx.amount
            )?;

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
