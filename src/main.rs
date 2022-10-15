const HELP: &str = "
USAGE:
    tinyvalue [FLAGS]

FLAGS:
    -c, --config     Config file path
";

struct Shard {
    name: String,
    index: i32,
    address: String,
    config: std::path::PathBuf,
    shard: String,
    replica: String
}

fn parse_flags(args: &Vec<String>) -> Result<&str, ()> {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config_path = parse_flags(&args).unwrap();
}
