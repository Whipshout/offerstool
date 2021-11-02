use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

use text_colorizer::Colorize;

pub fn read_file(path: String) -> Result<String, std::io::Error> {
    let now = Instant::now();

    let file = File::open(&path)?;
    let mut file_buf_reader = BufReader::new(file);
    let mut file_contents = String::new();
    file_buf_reader.read_to_string(&mut file_contents)?;

    let elapsed = now.elapsed().as_secs_f64();

    println!(
        "{} '{}': {:?}",
        "Time reading file".blue().bold(),
        path,
        elapsed,
    );

    Ok(file_contents)
}
