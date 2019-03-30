#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other:&Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
pub fn greeting(name: &str) -> String {
    format!("Hello!")
}
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
    
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        
        }

        Guess {
            value
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
   
     #[test]
    fn another() {
        assert_eq!(2 + 2, 4);
        //panic!("Make this test fail");
    }
    //Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
    }
    //Checking for Panics with should_panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        //cargo test -- --nocapture
        //for print show
       
        Guess::new(99);
    }
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }
    //Using Result<T, E> in tests
     #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
     #[test]
     #[ignore]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    //--running single tests
    // cargo test greater_than_100
    
    //--filtering to run multiple tests
    // cargo test it_
    
    //--ignoring some test
    //After #[test] we add the #[ignore]
    //If we want to run only the ignored tests, we can use cargo test -- --ignored:
}
