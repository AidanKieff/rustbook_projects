/* this function is bound to using it with one type, we can use the concept of genarica to
    make it useable with multiple types!
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

/*
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("the largest number is {result}"); 

    let char_list = vec!['y', 'm', 'a', 'z', 'q'];
    let result = largest(&char_list);
    println!("the largest char is {result}");


}
*/

//Structs

/*
struct Point<T> {
    x: T,
    y: T,
}
//here the struct is using genaric types, both x and y need to be the same type
//however different varaibles using the struct can be different types

fn main() {
    let integer = Point { x: 5, y: 10};
    let float = Point { x: 4.0, y: 10.1};

}
//if you want a struct with different genaric types per field, just need to add more genarics!
*/

/*
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 2.0, y: 3.3};
    let mixed = Point {x: 11, y: 5.5};
    let new = mixed.x + 1;
    println!("{new}");
}
*/

//method definitions
/*
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5, y: 10};

    println!("p.x = {}", p.x());
    
}
*/

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}
/*
this is to show that you can setup a method of a struct that also uses different types.
this method is an implamentation on the custom struct type so the impl needs that structs genaric types
but the method uses X2 and Y2, so they need to be on the method/function signature
*/

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: Point<X2, Y2>,
    ) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4};
    let p2 = Point { x: "hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}