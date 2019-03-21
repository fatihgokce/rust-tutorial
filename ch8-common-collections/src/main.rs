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
    println!("{:?}",v5);
    //Using an Enum to Store Multiple Types
    [std::fmt::Debug]
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
    println!("Hello, world!");
}
