use clap::ArgMatches;

pub struct Config {
    pub mixpanel_file: String,
    pub database_file: String,
    pub output_file: String,
}

impl Config {
    pub fn new(args: ArgMatches) -> Self {
        Config {
            mixpanel_file: args.value_of("mixpanel_file").unwrap().to_owned(),
            database_file: args.value_of("database_file").unwrap().to_owned(),
            output_file: args.value_of("output_file").unwrap().to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let n = 1;
        assert_eq!(n, 1);
    }
}
