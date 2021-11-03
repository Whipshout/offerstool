use std::fs::File;
use std::io::Read;
use std::time::Instant;

use text_colorizer::Colorize;

pub fn read_file(path: String) -> Result<String, std::io::Error> {
    let now = Instant::now();

    let mut file = File::open(&path)?;
    let file_len = file.metadata().unwrap().len();

    // let mut file_buf_reader = BufReader::new(file);
    // let mut file_contents = String::with_capacity(file_len as usize + 1);
    // file_buf_reader.read_to_string(&mut file_contents)?;
    let mut file_contents = Vec::with_capacity(file_len as usize + 1);
    file.read_to_end(&mut file_contents)?;

    // let mut contents_string = String::with_capacity(file_len as usize + 1);
    // file.read_to_string(&mut contents_string)?;

    let elapsed = now.elapsed().as_secs_f64();

    println!(
        "{} '{}': {:?}",
        "Time reading file".blue().bold(),
        path,
        elapsed
    );

    Ok(unsafe { String::from_utf8_unchecked(file_contents) })
}
