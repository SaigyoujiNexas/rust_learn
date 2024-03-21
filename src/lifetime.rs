use std::fmt::Display;

fn main(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next()
        .expect("Could not find first setence");
    let i = ImportantExcerpt{
        part: first_sentence
    };
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else {
        y
    }
}
 struct ImportantExcerpt<'a>{
    part:&'a str,
}
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,{
    println!("Announcement! {}", ann);
    if x.len() > y.len(){
        x
    } else {
        y
    }
}
