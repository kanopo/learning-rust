
struct User {
    username: String,
    age: u8,
    signed_in_counter: u64,
}


fn main() {

    let username = String::from("dmodmo");
    let age = 22;
    let counter = 0;
    
    let mut user = build_user(username, age, counter);

    println!("{:?}", user.username);
    println!("{:?}", user.age);
    println!("{:?}", user.signed_in_counter);

    user = increment_login(user);

    println!("{:?}", user.username);
    println!("{:?}", user.age);
    println!("{:?}", user.signed_in_counter);

}

fn build_user (usename: String, age: u8, counter: u64) -> User {
    let user1 = User {
        username: usename,
        age,
        signed_in_counter: counter
    };

    user1
}

fn increment_login(mut user: User) -> User {
    user.signed_in_counter += 1;

    user
}
