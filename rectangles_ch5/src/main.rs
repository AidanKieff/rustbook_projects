/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "the area of the rectangle is {} square pixels.", 
        area(&rect1)
    );

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

*/

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), //this is a macro that will print from stderror
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {:?}", rect1);

    println!(
        "the area of the rectangle is {} square pixels.", 
        area(&rect1)
    );

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //so far the above are methods as they need an instance of the datatype
    //to work with. these are asociated functions but specifically are methods 
    //they are used by setting the '.' on an instance of the data strucure
    //another way of knowing a functionn is a methos is the use of
    //the lowercase 'self' as a parameter

    //below is just an associated function, as it doesn't use an instance of the type

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
        //here the use of capitol 'Self' is an alias to the type that this function is under
        //specifically the type that the 'impl' block is referencing, aka Rectangle
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3); //associated fns that aren't methods use namespace/module syntax for calling, aka '::'
    println!(
        "sq is a new Rectangle structure created by the square associated function, and looks like this {:#?}", 
        sq
    );

    if rect1.width() {
        println!(
            "the rectangle has nonzero width; it is {}",
            rect1.width
        );
    }

    println!(
        "the area of the rectangle is {} square pixels.", 
        rect1.area()
    );

}

