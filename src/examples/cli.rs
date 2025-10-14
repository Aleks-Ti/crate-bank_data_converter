use std::io::{self, Cursor, Read, Write};

const LIMIT: usize = 100 * 1024 * 1024;

fn main() {
    let mut buffer = Vec::with_capacity(LIMIT);
    let mut stdin = io::stdin();

    let mut reader = stdin.take(LIMIT as u64);
    reader.read_to_end(&mut buffer)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {}

    println!("Выход из CLI, все изменения сохранены.");
}
