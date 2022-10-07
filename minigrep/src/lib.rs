use std::env;

pub struct Config<'a> {
    query: &'a String,
    filepath: &'a String,
    case_insensitive: bool
}

impl<'a> Config<'a> {
    pub fn parse(args: &Vec<String>) -> Result<Config, ()> {
        if args.len() < 3 {
            return Err(());
        }

        Ok(Config {
            query: &args[1],
            filepath: &args[2],
            case_insensitive: env::var("IGNORE_CASE").is_ok()
        })
    }
}

mod modules;
use modules::file;
use modules::search;

pub fn run(config: &Config) -> Result<(), &'static str> {
    let contents = file::read_file(config.filepath)?;
    // println!("{}", contents);
    let results = search::search(config.query, &contents, config.case_insensitive);
    for result in results {
        println!("{}", result);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parser() {
        let args: Vec<String> = vec![
                                    "minigrep".try_into().unwrap(),
                                    "search_query".try_into().unwrap(),
                                    "text.txt".try_into().unwrap()
                                ];
        let config = Config::parse(&args).unwrap();

        assert_eq!(config.filepath, "text.txt");
        assert_eq!(config.query, "search_query");
    }
}