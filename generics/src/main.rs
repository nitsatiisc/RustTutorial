

use generics::points::SimplePoint;

fn main() {
    let p:SimplePoint = SimplePoint::new(10,20);
    let dp = p + p;



    println!("Hello, world!");
    println!("Double of {} is {}", p ,dp);
}
