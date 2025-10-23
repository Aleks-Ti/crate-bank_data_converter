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
        let tx = txs.first().cloned().unwrap_or_else(|| Transaction {
            reference: "DEFAULT".to_string(),
            ..Default::default()
        });
        format!(
            r#"
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02">
  <BkToCstmrStmt>
    <Stmt>
      <Id>{}</Id>
      <Acct>
        <Id>{}</Id>
      </Acct>
      <Bal>
        <Amt Ccy="{}">{}</Amt>
      </Bal>
    </Stmt>
  </BkToCstmrStmt>
</Document>
"#,
            tx.reference, tx.account, tx.currency, tx.amount
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
