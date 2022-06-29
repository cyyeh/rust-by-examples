fn main() {
    /*
    if
    */
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let x = 6;

    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // variables must have a single type
    let y = if condition { 5 } else { 6 };
    println!("The value of number is: {}", y);

    /*
    loops

    Rust has three kinds of loops: loop, while, and for
    */
    // if you have loops within loops, break and continue apply to the innermost loop at that point
    // you can optionally specify a loop label on a loop that we can then use with break or continue
    // to specify that those keywords apply to the labeld loop instead of the innermost loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // conditional loops with while
    // this construct eliminates a lot of nesting that would be necessary
    // if you used loop, if, else, and break, and it's clearer
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    for (i, number) in (1..4).rev().enumerate() {
        println!("The {}th item is {}!", i+1, number);
    }
    println!("LIFTOFF!!!");
}
