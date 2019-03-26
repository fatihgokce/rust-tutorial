struct Point<T>{
    x:T,
    y:T,
}
impl<T> Point<T> {
    fn x(&self)->&T{
        &self.x
    }
}
struct Point2<T,U>{
    x:T,
    y:U,
}
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
   let number_list = vec![34, 50, 25, 100, 65];
   let f=Point2{x:1,y:2.1};

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    //In Method Definitions
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
//&Vec<i32>
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// fn largest<T>(list:&[T])->T{
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }