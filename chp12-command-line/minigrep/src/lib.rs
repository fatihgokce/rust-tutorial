use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    //&[String]                                 //String
     pub fn new(args: &Vec<String>) -> Result<Config,&'static str> {
        if args.len() < 3 {
            //panic!("not enough arguments");
            // return  Err(String::from("not enough arguments"));
            return  Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.filename)?;
    //.expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
    Ok(())
}