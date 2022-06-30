/*
This simple project demonstrates when we might want to use structs.
This program would take the width and height of a rectangle specified in pixels and calculate the area of the rectangle.
*/
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // refactoring with tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // refactoring with structs
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect2)
    );
    println!("rect2: {:?}", rect2);
    println!("rect2: {:#?}", rect2);
}

// The area function is supposed to calculate the area of one rectangle,
// but the function we wrote has two parameters, and it’s not clear anywhere
// in our program that the parameters are related.
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1.
// This would be even harder for someone else to figure out and keep in mind if they were to use our code.
// Because we haven’t conveyed the meaning of our data in our code, it’s now easier to introduce errors.
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}