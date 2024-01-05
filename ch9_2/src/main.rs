use std::arch::x86_64;
use std::fs::File;
use std::io::ErrorKind;

/*
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
*/


/*
fn main() {

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other => {
                panic!("Problem opening the file {:?}", other);
            }
        }
    };
}
*/

//using result type methods unwrap_or_else
/*
fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
*/

//using expect instead of unwrap


/*
fn main() {
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}
*/

//Propagating Errors


/*
//use std::fs::File;
use std::io::{self, Read}; 

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new(); 

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

*/

/*
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new(); 
    username_file.read_to_string(&mut username)?;
    Ok(username)

    /*
    can also chain: 
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username) 
    */
}
*/

//use std::num::ParseIntError; 
use std::error::Error; 
fn total_cost(item_quantity: &str) -> Result<i32, Box<dyn Error>> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty: i32 = item_quantity.parse()?;
    Ok(qty*cost_per_item + processing_fee)
}

fn main() {
    let x = total_cost("23000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    println!("{:?}", x);
    println!("{:?}", x.unwrap_err().to_string());
    
    
    
}