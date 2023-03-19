const HELP: &str = "
USAGE:
    tinyvalue [FLAGS]

FLAGS:
    -c, --config     Config file path
";

pub fn parse_flags(args: &Vec<String>) -> Result<&str, ()> {
    match args.len() {
        2 => {
            Ok(args[1].split("=").collect::<Vec<&str>>()[1])
        }
        _ => {
            println!("{HELP}");
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! str_vec {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    use super::*;

    #[test]
    fn proper_flags() {
        let short_flag = str_vec!["crate_path", "-c=config.ini"];
        let long_flag = str_vec!["crate_path", "--config=config.ini"];

        assert_eq!(parse_flags(&short_flag).unwrap(), "config.ini");
        assert_eq!(parse_flags(&long_flag).unwrap(), "config.ini");
    }

    #[test]
    #[should_panic]
    fn bad_flags() {
       let no_flag = str_vec!["crate_path"];
       let too_many_flags = str_vec!["crate_path", "-c=config.ini", "-other=other"];

       parse_flags(&no_flag).unwrap();
       parse_flags(&too_many_flags).unwrap();
    }
}