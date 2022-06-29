fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5 // or return 5;
}