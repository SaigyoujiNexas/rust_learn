fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let values = scores["Red"];
    let value = scores.get(&String::from("Blue"))
        .copied().unwrap_or(0);
    println!("{value}");

    for(key, value) in &scores{
        println!("{key}: {value}");
    }
    let text=  "hello world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
        
    
}
