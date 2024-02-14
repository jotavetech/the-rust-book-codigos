fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("joaozinho"),
        email: String::from("joaozinho@email.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);

    let username = String::from("mariazinha");

    let user2 = build_user(username, user1.email);

    println!("{}", user2.username);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: user2.email,
        sign_in_count: 2,
    };

    user1.username = String::from("joao2");

    let user4 = User {
        email: String::from("another@example.com"),
        ..user3
    };

    let black = Color(0, 0, 0);
    let mut white = Color(1, 1, 1);

    white = black;

    println!("White: {}", white.0);

    let teste = 1;
    let teste2 = teste;

    println!("Teste: {}, Teste2: {}", teste, teste2);

    let origin = Point(0, 0, 0);

    // black = origin; -> não pode, pois cada um tem seu tipo
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
