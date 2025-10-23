use clap::{Parser, ValueEnum};
use std::io::{self, Read};
use upload::upload;

const LIMIT: usize = 100 * 1024 * 1024; // 100 MiB

// cargo run --example cli -- --in-format csv --out-format mt940 -i "./example.csv" -o stdout "./example.mt940"
// cargo run --example cli -- --help
#[derive(Parser)]
#[command(version, about, long_about = "Cli инструмент для тестирования работы библиотеки.")]
struct Cli {
    /// Выберите формат ввода: "Csv", "mt940", "camt053",
    #[arg(short = 'I', long, value_enum)]
    in_format: Format,
    /// Выберите формат вывода данных: "csv", "mt940", "camt053",
    #[arg(short = 'O', long, value_enum)]
    out_format: Format,
    /// Опционально. При указании, ожидает путь к файлу. Пример: "path/to/file.format". Дефолтно - stdin()
    #[arg(short = 'i', long)]
    input: Option<String>,
    /// Опционально. При указании, ожидает путь куда будет сохранен файл. Пример: "path/to/file.format".  Дефолтно - stdout()
    #[arg(short = 'o', long)]
    output: Option<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Csv,
    Mt940,
    Camt053,
}

impl From<Format> for converter::Format {
    fn from(f: Format) -> Self {
        match f {
            Format::Csv => converter::Format::Csv,
            Format::Mt940 => converter::Format::Mt940,
            Format::Camt053 => converter::Format::Camt053,
        }
    }
}

fn read_input(path: Option<&str>) -> io::Result<Vec<u8>> {
    if let Some(filename) = path {
        let data = std::fs::read(filename)?;
        if data.len() > LIMIT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Файл {} слишком большой (>100 МБ)", filename),
            ));
        }
        Ok(data)
    } else {
        let mut buffer = Vec::with_capacity(LIMIT);
        let stdin = io::stdin();
        let mut limited = stdin.take(LIMIT as u64);
        limited.read_to_end(&mut buffer)?;

        if buffer.len() == LIMIT {
            let mut extra = [0u8; 1];
            if io::stdin().read_exact(&mut extra).is_ok() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "stdin превышает 100 МБ"));
            }
        }
        Ok(buffer)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let input_data = read_input(cli.input.as_deref())?;
    let output_text = converter::convert(
        &input_data[..],
        &converter::Format::from(cli.in_format),
        &converter::Format::from(cli.out_format),
    )?;

    upload(output_text.as_bytes(), cli.output.as_deref())?;
    Ok(())
}
