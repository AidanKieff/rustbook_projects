/*
pub fn add_two(a: i32) -> i32 {
    a+3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq! (4, add_two(2));
    }
}
*/

/*
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
    //intentionally fail test by removing name from being returned^^
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was '{result}'"
        //this below line is a custom message to deliver if the test fails
        );
    }
}
*/
/*
pub struct Guess {
    value: i32,
}
/*
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess value must be between 1 and 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}
*/
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be between 1 and 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}", 
                value
            );
        }
        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    //test that proves creating Guess instance outside the 1-100 range will panic the function
    //adding should panic attribute makes it to where a test will pass if the function running will panic
    //in parenthesis we are adding that we expect this substring to be apart of the function's panic!!
    fn greater_than_100() {
        Guess::new(200);
    }
}
*/

//using result<t, E> in Tests

pub fn add_two(a: i32) -> i32 {
    a+3
}

#[cfg(test)] 
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}