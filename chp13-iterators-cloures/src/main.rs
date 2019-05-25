use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::rt::v1::Count;

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    println!("Hello, world!");
    generate_workout(21,32);
    //move
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("can't use x here: {:?}", x);

}
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32,u32>,//Option<u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new()//None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v)=>v.clone(),
            None=>{
                let v=(self.calculation)(arg);
                self.values.insert(v,v);
                *v
            },
        }
//        match self.values.entry(arg.clone()) {
//            Entry::Occupied(v) => *v.into_mut(),
//            Entry::Vacant(v) =>   *v.insert((self.calculation)(arg)),
//        }
//        match self.value {
//            Some(v) => v,
//            None => {
//                let v = (self.calculation)(arg);
//                self.value = Some(v);
//                v
//            },
//        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
//    let expensive_closure = |num| {
//        println!("calculating slowly...");
//        thread::sleep(Duration::from_secs(2));
//        num
//    };
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
//        println!(
//            "Today, do {} pushups!",
//            expensive_closure(intensity)
//        );
//        println!(
//            "Next, do {} situps!",
//            expensive_closure(intensity)
//        );
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );

    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
//            println!(
//                "Today, run for {} minutes!",
//                expensive_closure(intensity)
//            );
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}