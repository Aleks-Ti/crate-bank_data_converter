# crate-bank_data_converter

## cli

Примеры команд:

```bash
cd parser
```

```bash
# справка по командам.
cargo run --example cli -- --help
```

```bash
# запуск с указанием путей
cargo run --example cli -- --in-format csv --out-format mt940 --input "./input_example.csv" --output "./output_example.mt940"
```

```bash
# запуск с указанием пути к таргетному файлу c выводом в stdout
cargo run --example cli -- -I csv -O mt940 -i "./input_example.csv"
```

```bash
# запуск передачей файла через stdin и выводом в stdout
cargo run --example cli -- -I csv -O mt940 -o "./output_example.csv"
```

## Данные

```csv
Id,ElctrncSeqNb,IBAN,Amount,Currency,CdtDbtInd,BookingDate,RemittanceInformation
40702810440000030888-1,1,40702810440000030888,1540.00,RUB,CRDT,2024-02-20,"Оплата по СЧЁТ № 4446141-5263495/NIC-D от 15.02.2024 по договору 5263495/NIC-DВ (регистрация доменного имени) В том числе НДС 20 % - 256.67 рублей"

```

```camt.053
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02">
  <BkToCstmrStmt>
    <Stmt>
      <Id>40702810440000030888-1</Id>
      <ElctrncSeqNb>1</ElctrncSeqNb>
      <Acct>
        <Id>
          <IBAN>40702810440000030888</IBAN>
        </Id>
      </Acct>
      <Ntry>
        <Amt Ccy="RUB">1540.00</Amt>
        <CdtDbtInd>CRDT</CdtDbtInd>
        <BookgDt>
          <Dt>2024-02-20</Dt>
        </BookgDt>
        <NtryDtls>
          <TxDtls>
            <RmtInf>
              <Ustrd>Оплата по СЧЁТ № 4446141-5263495/NIC-D от 15.02.2024 по договору 5263495/NIC-DВ (регистрация доменного имени) В том числе НДС 20 % - 256.67 рублей</Ustrd>
            </RmtInf>
          </TxDtls>
        </NtryDtls>
      </Ntry>
    </Stmt>
  </BkToCstmrStmt>
</Document>
```

```mt940
:20:40702810440000030888           // номер счета
:25:40702810600014448120           // счет корреспондента
:28C:1                            // номер выписки
:60F:C240220USD12345,67           // начальный остаток (пример)
:61:2402200220CR1540,00NMSCNONREF// дата, сумма, кредит (CR), код операции, ссылка
:86:Оплата по СЧЁТ № 4446141-5263495/NIC-D от 15.02.2024 по договору 5263495/NIC-DВ (регистрация доменного имени) В том числе НДС 20 % - 256.67 рублей
:62F:C240220USD13885,67           // конечный остаток (пример)
```
