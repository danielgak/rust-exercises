pub fn main() {
    let mut user = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    user.email = String::from("new email");

    let rect = Rectangle { x: 64, y: 100 };
    let area = rect.area();
    let area2 = calc_area(&rect);

    println!("{area} {area2}");
    println!("{user:?}");
    println!("{user:#?}");
    dbg!(&user); // linecode amended to log and extra info

    let area = Rectangle::square(13).area();

    let subject = AlwaysEqual;
    println!("{area} {subject:#?}");
}

// similar to a constructor
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// allowing to print debug
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    x: u64,
    y: u64,
}

// adding functions to a rectangle
impl Rectangle {
    fn area(&self) -> u64 {
        self.x * self.y
    }

    fn square(size: u64) -> Self {
        Self { x: size, y: size }
    }
}

fn calc_area(rect: &Rectangle) -> u64 {
    rect.x * rect.y
}

// unit like struct
#[derive(Debug)]
struct AlwaysEqual;
