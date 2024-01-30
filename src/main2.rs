fn main(){
    for i in 0..10 {
        println!("fibonacci({}) = {}", i, fibonacci(i));
    }
    let mut user  = User {
        active: true,
        username: "saigyoujinexas".to_string(),
        email: "saigyoujinexas@gmail.com".to_string(),
        sign_in_count: 0,
    };
    user.email = "".to_string();
    user  = User{
        email,
        username,
        sign_in_count: 1,
        active: false,
    };
    let mut user2 = User{
        active: user.active,
        username : user.username,
        email: String::from("saigyoujinexas@gmail.com"),
        sign_in_count: 1,
    };
    user2 = User{
        email: String::from("saigyouji"),
        ..user
    };
    let black = Color(0, 0, 0);

}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
    
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
