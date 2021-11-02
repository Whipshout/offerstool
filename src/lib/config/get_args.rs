use super::Config;
use clap::{App, Arg};

pub fn get_args() -> Config {
    let args = App::new("Templated offers tool")
        .version("1.0.0")
        .author("Eduardo S. <whipshout@gmail.com>")
        .about("Find common user_ids between two files and create a new file with these users_id and their card_ids")
        .arg(
            Arg::with_name("mixpanel_file")
                .help("Sets the mixpanel file to use")
                .default_value("mixpanel_dump.csv")
                .index(1),
        )
        .arg(
            Arg::with_name("database_file")
                .help("Sets the database file to use")
                .default_value("database_dump.csv")
                .index(2),
        )
        .arg(
            Arg::with_name("output_file")
                .help("Sets the output file to use")
                .default_value("output.csv")
                .index(3),
        )
        .get_matches();

    Config::new(args)
}
