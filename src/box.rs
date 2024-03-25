enum List {
    Cons(i32, Box<List>),
    Nil,
}
impl List {
    fn from_array(array: &[i32]) -> List {
        if let [head, tail @ ..] = array {
            List::Cons(*head, Box::new(List::from_array(tail)))
        } else {
            List::Nil
        }
    }
}
fn main() {
    let l = List::from_array(&[1, 2, 3, 4, 5, 6]);
}
