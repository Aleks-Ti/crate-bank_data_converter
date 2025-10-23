# crate-bank_data_converter

## cli

Примеры команд:

```bash
# справка по командам.
cargo run --bin cli -- --help
```

```bash
# запуск с указанием путей
cargo run --bin cli -- --in-format csv --out-format mt940 --input "./input_example.csv" --output "./output_example.mt940"
```

```bash
# запуск с указанием пути к таргетному файлу c выводом в stdout
cargo run --bin cli -- -I csv -O mt940 -i "./input_example.csv"
```

```bash
# запуск передачей файла через stdin и выводом в stdout
echo "ref,acc,desc
REF123,ACC456,Bread" | cargo run --bin cli -- --in-format csv --out-format mt940
```
