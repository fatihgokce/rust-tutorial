use std::collections::HashMap;
fn main() {
    let v:Vec<i32>=Vec::new();
    //Rust provides the vec! macro for convenience
    //creates a new Vec<i32> that holds the values
    let v2=vec![1,2,3];
    //Updating a Vector
    let mut v3=Vec::new();
    v3.push(5);
    // accessing a value in a vector, either with indexing syntax or the get method.
    let v4=vec![1,2,3,4,5];
    let third:&i32=&v4[2];
    println!("third element:{}",third);
    match v4.get(2) {
        Some(third)=>println!("third element with get:{}",third),
        None=>println!("there is no element"),
    }
    //Iterating over the Values in a Vector
    for i in &v4{
        print!("{}",i);
    }
    //To change the value that the mutable reference refers to,
    // we have to use the dereference operator (*)
    let mut v5=vec![10,20,30];
    for i in &mut v5{
        *i=*i*2;
    }
    //println!("{:?}",v5);
    //Using an Enum to Store Multiple Types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    //what is a string
    let mut s = String::from("foo");
    s.push_str("bar");
    //let s = format!("{}-{}-{}", s1, s2, s3);
    let mut hello = "çğü".to_string();
    for l in hello.bytes(){
        print!("{}-",l )
    }
    //Hash Map
    let mut scores=HashMap::new();
    scores.insert(String::from("Blue"),50);
    scores.insert(String::from("Red"),60);
    //get value return option
    //let score=scores.get(&String::from("Blue")).unwrap();
    //for
    for (key,value) in &scores{
        println!("{}:{}",key,value);
    }
    //updating a hash map
    scores.insert(String::from("Blue"), 25);
    //only inserting a value if the key has no value
    scores.entry(String::from("Yellow")).or_insert(51);
    println!("{:?}",scores);
    //Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);;
        *count += 1;
    }

    println!("{:?}", map);
    println!("Hello, world!");
}
