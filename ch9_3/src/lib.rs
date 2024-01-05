pub mod guessing {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value)
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::guessing::*;

    #[test]
    fn it_works() {
        let input = 40;
        let tester = Guess::new(input); 
        assert_eq!(input, tester.value());
    }

    #[test]
    #[should_panic]
    fn should_not_work() {
        let input = 500;
        let tester = Guess::new(input);
    }
}
