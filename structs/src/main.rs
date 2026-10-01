
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {

    // let user1 = User {
    //     active: true,
    //     username: String::from("user1name"),
    //     email: String::from("user1@gmail.com"),
    //     sign_in_count: 5,
    // };

    // let user2 = User {
    //     email: String::from("user2@gmail.com"),
    //     ..user1
    // };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )


}



// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 5,
//     }
// }