
struct Color{
    red:u8,
    green:u8
}
fn main() {
    const MAX_POINTS:u32=100_000;
    println!("Hello, world!");
    let (x,y,z)=(500,6.4,1);
    println!("{}",y);
    let a=[1,2,3];
    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    //functions
    let mut c =Color{red:122,green:23};
    print_to( &mut c);
    println!("{}",c.red);
    //control flow
    let number = if true {
        5
    }else{
        6
    };
    println!("number {}",number);
    //for
    let fa = [10, 20, 30, 40, 50];
    for element in fa.iter(){
        println!("the value {}",element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }

}
fn print_to(color:&mut Color){
    color.red=100;
    println!("{}",color.red);
}