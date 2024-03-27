fn main() {
    let person = Human;
    need_a_function(Pilot::fly);
}

fn need_a_function<F>(f: F)
    where
F:FnOnce(&Human)->(),
{
    let person = Human;
    f(&person);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}
struct Human;

impl Pilot for Human{
    fn fly(&self) {
        println!("THis is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
