//mod sound {
//    pub mod instrument {
//        pub fn clarinet() {
//            // Function body code goes here
//            //so we can use super to go to the parent module of instrument
//            super::breathe_in();
//        }
//    }
//    fn breathe_in() {
//
//        println!("callled breathe_in");
//    }
//}
mod sound;
mod animal{
    pub mod cat{
        pub fn lion(){
            println!("i'm lion");
        }
    }
}
mod sub;
mod math;

use crate::sound::instrument;
/*
If you want to bring an item into scope with use and a relative path,
there’s a small difference from directly calling the item using a relative path:
instead of starting from a name in the current scope,
you must start the path given to use with self.
*/
use self::animal::cat;
use crate::animal::cat::lion;
use crate::sound::instrument::clarinet;
use std::num::ParseIntError;

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
    println!("{}",math::math::add(21,45));
    println!("Hello, world!");
    println!("{}",sub::add(21,22));
    //use keyboard to bring paths
    clarinet();//absolute path
    lion();//relative path
    let s=Some(21);
    
    print(multiply("t", "2"));
    //print(multiply("t", "2"));
//    match s {
//        Some(i)=>println!("i {}",i+4),
//        _ =>println!("nothing")
//    }
}
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    println!("first:{}",first_number);
    println!("second:{}",second_number);
    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
