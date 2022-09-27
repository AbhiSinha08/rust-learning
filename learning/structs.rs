#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    logins: u64
}

fn make_user(username: &str, email: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        active: true,
        logins: 1
    }
}

fn main() {
    let user1 = User {
        username: String::from("abhinavkumar"),
        email: String::from("abhinav@gmail.com"),
        active: true,
        logins: 1
    };

    dbg!(&user1);

    let user2 = make_user("username2", "emailid");
    dbg!(&user2);
    let user3 = User {
        username: String::from("username2"),
        ..user1
    };
    dbg!(&user3);
}