use std::ops::Add;

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}
struct OtherPoint<T, U>{
    x: T,
    y: U,
}

impl <T: Add + Copy> OtherPoint<T, T>{
    fn sum(&self) -> T::Output  {
        self.x + self.y
    }
}
fn main(){
    let number_list = vec![34, 50, 25, 100, 65];
    let integer = Point{ x: 5, y : 10};
    println!("{:?}", integer);
    let float  = Point{x: 1.0, y: 10.0};

}

fn largest<T : PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}
