
/*
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
    Texas,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn main() {
    let test = value_in_cents(Coin::Quarter(UsState::Alaska));

}
*/


/*
 fn main() {
    //let z = plus_one("five"); << will not work, is not an i32
    let six = Some(6);
    let y = plus_one(six);

    println!("{:?}", y); 
 }
 fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

}
*/

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
    //other is a catch all that matches if the first arms don't get run. it will use the value though!

    //for something that is a catchall that won't use the value:
    // _ => reroll(),
    //or
    // _ => () in which no code will run at all
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}