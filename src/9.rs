use std::fmt;

fn main() {
    println!("Hello, world!");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut users_string = String::new();
        for user in self.users.iter() {
            if user.active {
                users_string.push_str(&format!("  - {} [{}](mailto:{})\n", user.username, user.email, user.email));
            } else {
                users_string.push_str(&format!("  - {} ({}) [{}](mailto:{})\n", user.username, user.sign_in_count, user.email, user.email));
            }
        }
        write!(f, "Users:\n{}\n", users_string)
    }
}

fn main() {
    let mut users = vec![];
    for i in 0..10 {
        users.push(User {
            username: format!("user{}", i),
            email: format!("user{}{}", i, "@example.com"),
            sign_in_count: i as u64,
            active: i % 2 == 0,
        });
    }
    println!("{}", UserList { users });
}
