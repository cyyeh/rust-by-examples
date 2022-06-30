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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// multiple impl blocks is a valid syntax, but no reason to separate these methos into multiple impl blocks here
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    /*
    We’ve chosen &self here for the same reason we used &Rectangle in the function version:
    we don’t want to take ownership, and we just want to read the data in the struct, not write to it.
    If we wanted to change the instance that we’ve called the method on as part of what the method does, 
    we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by
    using just self as the first parameter is rare; this technique is usually used when the method transforms self
    into something else and you want to prevent the caller from using the original instance after the transformation.
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

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

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect)
    );
    println!("rect: {:?}", rect);
    println!("rect: {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );
    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("square: {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}