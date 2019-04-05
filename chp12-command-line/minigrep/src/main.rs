use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
//    for i in &args{
//        println!("{}",i);
//    }
    let config=minigrep::Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
//    println!("Searching for {}", config.query);
//    println!("In file {}", config.filename);
//    let contents = fs::read_to_string(config.filename)
//           .expect("Something went wrong reading the file");
//    if let Err(e) = run(config) {
//        println!("Application error: {}", e);
//
//        process::exit(1);
//    }
    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }

}
fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
        //.expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
    Ok(())
}
//struct Config {
//    query: String,
//    filename: String,
//}
//impl Config {
//    //&[String]                                 //String
//    fn new(args: &Vec<String>) -> Result<Config,&'static str> {
//        if args.len() < 3 {
//            //panic!("not enough arguments");
//            // return  Err(String::from("not enough arguments"));
//            return  Err("not enough arguments");
//        }
//        let query = args[1].clone();
//        let filename = args[2].clone();
//        Ok(Config { query, filename })
//    }
//}
//fn parse_config(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let filename = args[2].clone();
//    Config{query,filename}
//}