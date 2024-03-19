fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    let third = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),

        None => println!("There is no third element."),

    }

    let j = v.get(5);
    j.map_or(-1,  |x| x * 2);

    let v = vec![100, 32, 57];
    for i in &v{
        println!("{i}");
    }

    //change
    let mut v = vec![100, 32, 57];
    for i in &mut v{
        *i += 50;
    }
}
