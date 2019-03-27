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
//10.2 trait
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
//for default implemantion
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// //Trait as arguments
fn notify(item:& impl Summary) {
    println!("Breaking news! {}", item.summarize_author());
}
fn notify2<T: Summary>(item: &T) {
    println!("2Breaking news! {}", item.summarize_author());
}
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
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
    //10.2 Trait
    let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };
    let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
    };
    
    println!("New article available! {}", article.summarize());
     //Trait as arguments
    notify(&tweet);
    notify(&article);
    //Trait Bounds
    notify2(&tweet);
    /*
    fn notify(item1: impl Summary, item2: impl Summary) {
    //same
    fn notify<T: Summary>(item1: T, item2: T) { 
    */
    // Specify nulttipe traits with +
    /*
    fn notify(item: impl Summary + Display) {
        //with generic
    fn notify<T: Summary + Display>(item: T) {
     */
    // where clauses for clearer code
    /*
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    //we can use a where clause, like this:
    fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
    {
    */
    //Returning Traits
    /*
    This signature says, “I’m going to return something that implements the Summary trait, but I’m not going to tell you the exact type.” 
    */
    let rt=returns_summarizable();
    println!("{}",rt.summarize());

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