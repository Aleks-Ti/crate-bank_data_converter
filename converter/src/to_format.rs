use crate::model::Transaction;

pub trait ToFormat {
    fn from_transactions(txs: &[Transaction]) -> String;
}

pub struct CsvFormat;
impl ToFormat for CsvFormat {
    fn from_transactions(txs: &[Transaction]) -> String {
        let mut output = "reference,account,amount,currency,date,description\n".to_string();
        for tx in txs {
            output.push_str(&format!(
                "{},{},{},{},{},{}\n",
                tx.reference, tx.account, tx.amount, tx.currency, tx.value_date, tx.description
            ));
        }
        output
    }
}

pub struct Mt940Format;
impl ToFormat for Mt940Format {
    fn from_transactions(txs: &[Transaction]) -> String {
        let mut output = String::new();
        for tx in txs {
            output.push_str(&format!(":20:{}\n", tx.reference));
            output.push_str(&format!(":25:{}\n", tx.account));
            output.push_str(&format!(
                ":61:{}{}{}DR{:.2}NMSCNONREF\n",
                &tx.value_date[2..4],
                &tx.value_date[5..7],
                &tx.value_date[8..10],
                tx.amount
            ));
            if !tx.description.is_empty() {
                output.push_str(&format!(":86:{}\n", tx.description));
            }
            output.push('\n');
        }
        output
    }
}

pub struct Camt053Format;
impl ToFormat for Camt053Format {
    fn from_transactions(txs: &[Transaction]) -> String {
        if txs.is_empty() {
            return r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02"></Document>"#.to_string();
        }

        let statement_id = &txs[0].reference;
        let account_id = &txs[0].account;

        let entries: Vec<String> = txs
            .iter()
            .map(|tx| {
                format!(
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
                )
            })
            .collect();

        format!(
            r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02">
            <BkToCstmrStmt>
                <Stmt>
                <Id>{}</Id>
                <Acct>
                <Id>{}</Id>
                </Acct>
                {}
                </Stmt>
            </BkToCstmrStmt>
            </Document>
            "#,
            statement_id,
            account_id,
            entries.join("\n")
        )
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

    let output = Camt053Format::from_transactions(&txs);
    assert!(output.contains("<Ntry>"));
    assert_eq!(output.matches("<Ntry>").count(), 2);
    assert!(output.contains("100.5"));
    assert!(output.contains("200"));
}
