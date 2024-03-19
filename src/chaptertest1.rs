use std::collections::{HashMap, VecDeque};
fn main(){
    test2();
}


fn test1(){
    let vec = &mut [10, 2, 100, 2, 39, 65, 21, 15, 100, 100, 10, 2, 2];
    vec.sort();
    let mut map = HashMap::new();
    for i in vec.iter(){
        let cnt = map.entry(i).or_insert(0);
        *cnt += 1;
    }
    let mut max_cnt = 0;
    let mut mode = 0;
    for(value, cnt) in map{
        if cnt > max_cnt {
            max_cnt = cnt;
            mode = *value;
        }
    }

    println!("the middle number is {}, and mode is {}",
    vec.get(vec.len() >> 1).unwrap(), mode);
}


fn test2(){
    let str = "Hello, Android";
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut ans = str.split_whitespace()
        .fold(String::new(), |mut acc, word|{
            let mut chars = word.chars();
            let first = chars.next().unwrap();
            let new_word = if vowels.contains(&first.to_ascii_lowercase()) {
                format!("{word}-hay ")
            } else {
                format!("{}-{first}ay ", chars.as_str())
            };
            acc.push_str(&new_word);
            acc
        });
    ans.pop();
    println!("{ans}");
}

struct Company{
    departments: HashMap<String, Vec<String>>
}

impl Company {
    fn new() -> Company {
        Company{
            departments: HashMap::new()
        }
    }
    fn add_department(&mut self, department_name: &str) -> Option<()>{
        if self.departments.contains_key(department_name){
            return None;
        }
        self.departments.insert(department_name.to_string(), Vec::new());
        Some(())
    }
    fn add_employee(&mut self, name: &str, department_name:&str)->Option<()>{
        let department = self.departments.get_mut(department_name);
        match department {
            Some(value) => {
                (*value).push(name.to_string());
                Some(())
            }
            None => {
                None
            }
            
        }
    }
    fn list_employees(&mut self, department_name: &str) -> Vec<&str>{
        let mut result = self.departments.get(department_name).into_iter()
            .flatten()
            .map(String::as_str)
            .collect::<Vec<_>>();
        result.sort();
        result
    }

    fn list_all_employees(&self) -> Vec<&str>{
        let mut res: Vec<&str> = self.departments.iter()
            .flat_map(|(_, v)|{
                v.iter()
            })
            .map(String::as_str)
            .collect();
        res.sort();
        res
    }

}


















