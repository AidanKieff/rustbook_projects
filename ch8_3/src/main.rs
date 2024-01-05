use std::collections::{HashMap, btree_map::VacantEntry};


/*
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}:{value}");
    }
}
*/

/*
fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //original variables now dropped as they were strings. the values 
    //have been moved into the hash table.

}
*/

//overwriting a value in the hash table
/*
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    //overwriting value by calling same key with new value
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

}
*/

//adding k:v only if a key isn't present

/*
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    //entry method will almost always want to be called with or_insert
    //this way, if there is not that enter, it will insert the key from
    //the entry param, as well as a value from the or_insert param
    scores.entry("Yellow".to_string()).or_insert(50);
    //in the below, there is already a blue key, so it will not update
    //the value of key blue
    let x = scores.entry("Blue".to_string()).or_insert(50);
    *x = 100;
    //weirdly, if we wanted to update a value, we could just fully overwrite as in 
    //the above main function, or call and_modify on the entry call:::
    //scores.entry("Blue".to_string()).and_modify(|v| { *v = 50} );
    //and modify is a function on the Entry enum 
    //println!("{:?}", scores.entry("Black".to_string()).or_insert());
    //println!("{:?}", scores.entry("Blue".to_string()).or_insert(4));
    println!("{:?}", scores);
    let y = scores.entry("Green".to_string());
    println!("{:?}", y);


}
*/

//Updating a value based on the olld value

fn main() {
    let text = "hello the world the wonderful wonderful world wide world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        
        let count = map.entry(word).or_insert(0);
        //does the same thing as above 
        //let count: &mut u8 = map.entry(word).or_default();
        
        *count += 1;
        //
        //the below covers this all in one line:
        //let count = map.entry(word).and_modify(|e| {*e += 1}).or_insert(1);
        //
    }

    println!("{:?}", map);
}

fn Main () {
    let text = "hello the world the wonderful wonderful world wide world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        
        let count = map.entry(word).or_insert(0);
        //does the same thing as above 
        //let count: &mut u8 = map.entry(word).or_default();
        
        *count += 1;
        //
        //the below covers this all in one line:
        //let count = map.entry(word).and_modify(|e| {*e += 1}).or_insert(1);
        //
    }

    println!("{:?}", map);
}