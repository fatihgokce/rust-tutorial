use serde_json::json;
fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!("first phone number:{}",john["phones"][0]);
    println!("{}",john.to_string());
    println!("Hello, world!");
}
