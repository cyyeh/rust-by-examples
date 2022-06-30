/*
we used the owned String type rather than the &str string slice type.
This is a deliberate choice because we want each instance of this struct to own all of its data and
for that data to be valid for as long as the entire struct is valid.
*/
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs without named fields
struct Color(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

// For structs to store references to data owned by something else would be discussed in lifetimes.
// Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("email: {}", user1.email);

    let user2 = build_user(String::from("test@gmail.com"), String::from("test"));
    println!("name: {}", user2.username);

    let user3 = User {
        email: String::from("test2@gmail.com"),
        ..user1 // this should come last
    };
    // error, user3 is immutable by default
    // user3.email = String::from("test3@gmail.com");
    println!("email: {}", user3.email);
    // error, since user1 moves the data, and type of user1.username doesn't implement the Copy trait.
    // println!("name: {}", user1.username);

    let black = Color(0, 0, 0);
    println!("({}, {}, {})", black.0, black.1, black.2);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}