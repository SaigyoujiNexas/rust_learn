use std::fmt::{Display, Debug};

trait Summary {
    fn summary(&self) -> String;
mod hashmap;

fn main(){
    //slice str is &str type.
    //constant string is the slices of string.
    //
    // create a new empty string.
    let mut s  = String::new();
    //use to_string method create String from a constant string.
    let data = "innitial contents"; //data: &str.
    let s = data.to_string();       //s: String.

    let s = "initial contents".to_string();
    //same as to_string method.
    let s = String::from("initial contents");
    //String is utf-8 coding, so it can include any data that have correct code.
    //
    let hello = String::from("koko");
    let hello = String::from("在这里");

    //update String
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}, s2 is {s2}");
    let mut s = String::from("lo");
    s.push('l');
    s += "test";

    //can use + or format! to combine string.
    let s1 = String::from("Hello, ");
    let s2= String::from("world");
    let s3 = s1 + &s2;

    //use format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    //string index.
    let s1 = String::from("hello");
    //unsafe.
    let h = &s1[0..4];

    //transerve string.
    for c in "测试".chars() {
        println!("{c}");
    }

    //origin byte transerve
    for b in "测试".bytes() {
        println!("{b}");
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary){

}

//equals
fn notify2<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summary());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
T: Display + Clone,
    U:Clone + Debug,
{
    return 0;
}

