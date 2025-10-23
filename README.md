# crate-bank_data_converter

Консольная утилита и библиотека для конвертации банковских выписок между форматами:

- **CSV** — простой табличный формат
- **MT940** — SWIFT-подобный формат выписок
- **CAMT.053** — XML-формат по стандарту ISO 20022

Проект состоит из нескольких крейтов:

- [`parser`](./parser/src/lib.rs) — парсинг и сериализация форматов
- [`converter`](./converter/src/lib.rs) — конвертация между форматами
- [`upload`](./upload/src/lib.rs) — запись данных в файл или stdout
- [`cli`](./cli/src/main.rs) — консольный интерфейс

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
