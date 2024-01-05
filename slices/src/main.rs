/*
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); //word will get the vlue 5
    println!("{word}");
    println!("{s}");
    s.clear(); 

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
*/

/*
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); //word will get the vlue 5
    println!("{word}");
    let word2 = second_word(&s);
    println!("{word2}");
    println!("{s}");
    s.clear(); 

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }

    &s[..]
}
*/
