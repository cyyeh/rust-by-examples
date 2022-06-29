fn main() {
    let s1 = String::from("hello");
    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference stops being used.
    // We call the action of creating a reference borrowing. As in real life, if a person owns something,
    // you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // there would be a compile error, since s2 is immutable (by default), and so is its reference
    // let s2 = String::from("hello");
    // change(&s2);
    // println!("s2: {}", s2);

    let mut s3 = String::from("hello");
    change2(&mut s3);
    println!("s3: {}", s3);
    // Mutable references have one big restriction:
    // you can have only one mutable reference to a particular piece of data at a time.
    // the code below will fail
    // let r1 = &mut s3;
    // let r2 = &mut s3;
    // println!("{}, {}", r1, r2);

    /*
    The restriction preventing multiple mutable references to the same data at the same time allows for mutation but
    in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you
    mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile
    time. A data race is similar to a race condition and happens when these three behaviors occur:
    1. Two or more pointers access the same data at the same time.
    2. At least one of the pointers is being used to write to the data.
    3. There’s no mechanism being used to synchronize access to the data.

    Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down
    at runtime; Rust prevents this problem by refusing to compile code with data races!

    As always, we can use curly brackets to create a new scope, allowing for multiple mutable references,
    just not simultaneous ones
    */
    let mut ss = String::from("hello");
    {
        let r1 = &mut ss;
        println!("r1: {}", r1);
    } // r1 goes out of the scope here

    let r2 = &mut ss;
    println!("r2: {}", r2);

    // Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
    // let mut ss2 = String::from("hello");
    // let rr1 = &ss2; // no problem
    // let rr2 = &ss2; // no problem
    // let r3 = &mut ss2; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // However, the code below is ok
    let mut ss2 = String::from("hello");

    let rr1 = &ss2; // no problem
    let rr2 = &ss2; // no problem
    println!("{} and {}", rr1, rr2);
    // variables r1 and r2 will not be used after this point

    let rr3 = &mut ss2; // no problem
    println!("{}", rr3);

    // dangling references
    // let reference_to_nothing = dangle();

    // no dangling
    let no_dangling = no_dangle();
    println!("{}", no_dangling);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// } // Here s goes out of scope, and is dropped. Its memory goes away.

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // This works without problems. Ownership is moved out, and nothing is deallocated.