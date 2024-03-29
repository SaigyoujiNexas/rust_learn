use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmatPointer{
    data: String,
}

impl Drop for CustomSmatPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmatPointer with data `{}`!", self.data);
    }
}

fn main() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);
    let a = 3;
    let b = MyBox::new(a);
    assert_eq!(3, *b);
    let c = CustomSmatPointer{
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmatPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmatPointer created");
}
