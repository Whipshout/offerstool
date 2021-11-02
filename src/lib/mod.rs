use std::collections::HashSet;
use std::time::Instant;

use text_colorizer::Colorize;

use crate::config::Config;
use crate::io::{read_file, write_file};
use crate::utils::format_data;

pub mod config;
pub mod io;
pub mod utils;

pub fn run(config: Config) -> Result<(), std::io::Error> {
    let now_total = Instant::now();

    let mixpanel_contents = read_file(config.mixpanel_file)?;
    let mixpanel: HashSet<&str> = mixpanel_contents.lines().into_iter().collect();

    let database_contents = read_file(config.database_file)?;
    let database: Vec<&str> = database_contents.lines().into_iter().collect();

    let (report_text, report_count) = format_data(mixpanel, database);

    write_file(config.output_file, report_text, report_count)?;

    let elapsed_total = now_total.elapsed().as_secs_f64();

    println!(
        "{} {:?}",
        "Total time used in seconds:".yellow().bold(),
        elapsed_total
    );

    Ok(())
}
