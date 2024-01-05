
/*
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
*/
/*/
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

}

fn calculate_length(s: &String) -> usize {
    s.len()
}
*/

fn main() {
    let mut s = String::from("hello");
    change (&mut s);

    println!("{s}"); //after print, reference goes out of scope but s is still in scope because we were only referencing

    let mut s2 = s; //can now bind value of s to new mutable varable and run the process again

    change (&mut s2);

    println!("{s2}"); //s2 therefore all values now dropped from memory
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}