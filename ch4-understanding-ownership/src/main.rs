struct Color {
    red: u8,
    green: u8,
}

fn main() {
    let s1 = String::from("hello");

    let (mut s2, len) = calculate_length(s1);
    let _ = calculate_length2(&mut s2);
    let s3: &String = &s2;
    println!("The length of '{}' is {}.s2 is {}", s2, len, s3);
    //4.3 slices
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5

    //s.clear(); // this empties the String, making it equal to ""
    //println!("{} {}",word,s);
    //but not including, end. If we wanted to include end,
    // we can use ..= instead of ..:
    let hello = &s[0..5];
    let world = &s[6..11];
    //With Rust’s .. range syntax, if you want to start at the first index (zero),
    //you can drop the value before the two periods. In other words, these are equal:
    let slice = &s[0..2];
    let slice = &s[..2];
    //By the same token, if your slice includes the last byte of the String,
    //you can drop the trailing number. That means these are equal:
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    let slice = &s[0..len];
    let slice = &s[..];

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word2(&my_string[..]);
    println!("{}", word);
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word2(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word2(my_string_literal);

    //Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    let a1 = Color { red: 122, green: 23 };
    let b2 = a1;
    println!("{}", a1.green);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}