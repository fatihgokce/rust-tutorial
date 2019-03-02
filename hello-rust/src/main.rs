use std::mem;


fn main() {
    let t=(1,'a',false);
    println!("Hello, world! {}",t.0);
    //debug flag
    //print!("{:?}",t);
    //arrays
    let  xs:[i32;5]=[4,5,6,7,8];
    print!("{}:{}:{}",xs[0],xs.len(),mem::size_of_val(&xs));
    let  ys=&xs[2..4];
    print!("{:?}",ys);
    //string
    let mut s="String";
    let ss=String::from("String");



}
