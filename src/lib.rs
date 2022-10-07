use std::{ error::Error, fs };

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 表示发生错误不会恐慌(panic),而是返回错误值
    let contents: String = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            query,
            filename,
        })
    }
}