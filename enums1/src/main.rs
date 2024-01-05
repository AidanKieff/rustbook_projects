/*
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
}

// fn route(ip_kind: IpAddrKind) {} 
// this is an example of the fact this enum structure
// can be used in a function and the enum name is the type

*/

/*
enum IpAddr {
    V4(String),
    V6(String), //enum 'IpAddr::V6()' is a function call that takes a string
                //and returns that string value and is an IpAddr type 
}
// the above shows how you can add associated values to enum variants

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));    
}
*/

/*
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

}
*/

/*
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body would be defined here
    }
}

fn main() {
    let m = Message::Write(Strinf::from("hello"));
    m.call(); 
}
*/

