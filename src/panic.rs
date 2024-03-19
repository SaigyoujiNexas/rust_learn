use core::panic;
use std::{fs::{File, OpenOptions}, io::{Read, ErrorKind, Write, self}};

fn main(){

    let file_name = "hello.txt";
    let file = OpenOptions::new().write(true).open(file_name);
    let mut f = match file {
        Ok(ofile) => ofile,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("..... {:?}", e),
            }
            other_error =>{
                panic!("...... {:?}", other_error)
            }
        }
        
    };
    let result = f.write("Hello, world".as_bytes());

    match result {
        Ok(size) => println!("written {size} bytes."),
        Err(error) => println!("written {error}"),
    }
}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username).map(|_| username)
}
