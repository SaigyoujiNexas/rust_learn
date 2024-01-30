use std::fmt::{Display, Debug};

trait Summary {
    fn summary(&self) -> String;
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
fn main(){

}

