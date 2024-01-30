fn main(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    //type inference
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("yellow"), 50);

    //visit value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

}
