use std::collections::HashSet;
use std::fmt::Write;
use std::time::Instant;

use text_colorizer::Colorize;

pub fn format_data(mut mixpanel: HashSet<&str>, database: Vec<&str>) -> (String, u32) {
    let now = Instant::now();

    let mut report_text = String::new();
    let mut report_count: u32 = 0;

    for data in database.into_iter() {
        if data.len() == 73 {
            let (user_id, card_id) = data.split_once(';').unwrap();
            if mixpanel.contains(user_id) {
                mixpanel.remove(user_id);
                writeln!(report_text, "{};{}", user_id, card_id).unwrap();
                report_count += 1;
            }
        }
    }

    let elapsed = now.elapsed().as_secs_f64();

    println!("{} {:?}", "Time formatting file:".red().bold(), elapsed);

    (report_text, report_count)
}
