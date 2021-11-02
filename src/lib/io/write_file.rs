use std::fs::File;
use std::io::Write;
use std::time::Instant;

use text_colorizer::Colorize;

pub fn write_file(path: String, text: String, count: u32) -> Result<(), std::io::Error> {
    let now = Instant::now();

    let mut buffer = File::create(path)?;
    buffer.write_all(text.as_bytes())?;

    let elapsed = now.elapsed().as_secs_f64();

    println!(
        "{}{}): {:?}",
        "Time writing file (Rows: ".blue().bold(),
        count,
        elapsed,
    );

    Ok(())
}
