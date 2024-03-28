use std::{alloc::System, process::exit};

fn main() {
    let person = Human;
    need_a_function(Pilot::fly);
    do_twice(add_one, 5);
}

fn need_a_function<F>(f: F)
where
    F: FnOnce(&Human) -> (),
{
    let person = Human;
    f(&person);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("THis is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

type Kilometers = i32;
fn type_def() {
    let y: Kilometers = 3;
}
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}
fn bar() -> ! {
    //--snip --
    exit(1);
}

fn generic<T: ?Sized>(t: &T) {}
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
