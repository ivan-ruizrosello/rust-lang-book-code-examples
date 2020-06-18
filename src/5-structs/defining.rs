fn main() {
    let user1 = build_user(
        String::from("ivan.ruizrosello@gmail.com"),
        String::from("ivanruizdev"),
    );

    print_user(&user1);

    let user2 = User {
        email: String::from("mail2@mailing.es"),
        ..user1
    };

    print_user(&user2);


    let black = Color(0,0,0);
}
struct Color(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(u: &User) {
    println!(
        "
    username: {},
    email: {},
    sign_in_count: {},
    active: {}",
        u.username, u.email, u.sign_in_count, u.active
    );
}
