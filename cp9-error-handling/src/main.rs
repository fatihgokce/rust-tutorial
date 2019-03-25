use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;
use std::fmt::Error;

fn main() {

    let f = File::open("hello.txt");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    //That’s a lot of match! match is very powerful,
    // but also very much a primitive.
    // A more seasoned Rustacean might write this
    let f = File::open("hello.txt").map_err(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        }else{
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
    //Shortcuts for Panic on Error: unwrap and expect
    let f = File::open("hello.txt").unwrap();
    //Another method, expect, which is similar to unwrap,
    // lets us also choose the panic! error message.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    //Propagating Errors
    let f=read_username_from_file();
    ////A Shortcut for Propagating Errors: the ? Operator
    /*
    File::create("foo.txt")?.write_all(b"Hello world!")
    //Would be transformed to:
    match File::create("foo.txt") {
    Ok(t)  => t.write_all(b"Hello world!"),
    Err(e) => return Err(e.into()),
}
    */
    //The ? Operator Can Only Be Used in Functions That Return Result
    //However, the main function can return a Result<T, E>:
    /*
    //you can read Box<dyn Error> to mean “any kind of error.”
    fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
    */
    let f=read_username_from_file2();
    println!("Hello, world!");
    //panic!("crash and burn");
    //9.3 panic! or not to panic!
    
    
    
}
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
//A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
