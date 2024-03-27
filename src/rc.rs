enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //will inc strong.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let x = RefCell::new(vec![1, 2, 3, 4, 5]);
    x.borrow_mut().push(3);
    circle_reference();
    weak_rc();
}

#[derive(Debug)]
enum RefList {
    Cons(i32, RefCell<Rc<RefList>>),
    Nil,
}
impl RefList {
    fn tail(&self) -> Option<&RefCell<Rc<RefList>>> {
        match self {
            RefList::Cons(_, item) => Some(item),
            RefList::Nil => None,
        }
    }
}

fn circle_reference() {
    let a = Rc::new(RefList::Cons(5, RefCell::new(Rc::new(RefList::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(RefList::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_rc() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}
