/*

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("the first element is {first}");
    // will not compile because trying to change memory while holding an element in 'first' variable

}
*/

/*

fn main() {
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
        //dereferencing (*) in order to mess with the actual values in the referencing of v (&mut v)
    }
}
*/

//when we need one type to rep elements of different types, this is when enums come in! >>

fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    //vectors can only store one type. well an enum is a single type! 
    
}

