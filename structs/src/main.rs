struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someguy"),
        email: String::from("somone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anothersomeone@email.com"); 

    let user2 = User {
        active: user1.active,
        username: user1.username, //because we move this string value, user1 is no longer valid
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user22 = User {
        email: String::from("yetanother@example.com"),
        ..user2 //can't use user1 because string values were moved to user2 
                //other than that, this is just to show a quicker way of copying over values used in a previous struct creation
    };

    

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //can write like this uses init shorthand: the function paremeter is the same name as the field so rust just knows when writing this way
        email: email,
        sign_in_count: 1,
    }
}