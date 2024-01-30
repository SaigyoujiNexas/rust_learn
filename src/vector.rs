fn main(){
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.pop();
    
    let mut v2 = vec![1, 2, 3];
    v.append(&mut v2);
    for e in v.iter() {
        println!("{e}");
    }
    for e in v.iter() {
        println!("{e}");
    }
    for i in &mut v {
        *i ++ 50;
    }
    println!("v: {}, v2: {}", v.len(), v2.len());
    let v3 = vec!["Hello", "World"];
    let i = v.get(0);
    println!("{}", i.unwrap());
    let j = v.get(5);
    let jj = i.map_or(-1,  |x| x * 100);
    match j {
        Some(value) => println!("{value}"),
        None => println!("j is none"),
    }
    println!("{}", jj);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec!{
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    };
    for e in &row{
        match e {
            SpreadsheetCell::Int(i) => println!("{i}"),
            SpreadsheetCell::Text(s) => println!("{s}"),
            SpreadsheetCell::Float(f) => println!("{f}"),
        }
    }
}
