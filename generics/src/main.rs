use generics::lifetimes::VecVec;
use generics::points::SimplePoint;

fn main() {
    let p:SimplePoint = SimplePoint::new(10,20);
    let dp = p + p;



    println!("Hello, world!");
    println!("Double of {} is {}", p ,dp);

    let mut vv = VecVec::new();
    let a:Vec<u32> = vec![0,1,2];
    let b:Vec<u32> = vec![3,4,5];
    vv.add_vec(&a);
    vv.add_vec(&b);




}
