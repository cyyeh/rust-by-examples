/*
Ownership is a set of rules that governs how a Rust program manages memory.
All programs have to manage the way they use a computer’s memory while running.
Some languages have garbage collection that constantly looks for no-longer used memory as the program runs;
in other languages, the programmer must explicitly allocate and free the memory.
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.
If any of the rules are violated, the program won’t compile.
None of the features of ownership will slow down your program while it’s running.
Because ownership is a new concept for many programmers, it does take some time to get used to.
The good news is that the more experienced you become with Rust and the rules of the ownership system,
the easier you’ll find it to naturally develop code that is safe and efficient. Keep at it!
When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique.
In this chapter, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap,
and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
Once you understand ownership, you won’t need to think about the stack and the heap very often,
but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

Ownership Rules
First, let’s take a look at the ownership rules.
Keep these rules in mind as we work through the examples that illustrate them:
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    let s = "";
    let mut s2 = String::from("hello");
    {
        let s = "hello";
        s2.push_str(", world!");
        println!("s: {}", s);
        println!("s2: {}", s2);
    }
    println!("s: {}", s);
    println!("s2: {}", s2);

    let ss1 = String::from("hello");
    let ss2 = ss1; // ss1 is moved, ss1 is invalidated
    println!("ss2: {}", ss2);
    // println!("{}, world!", ss1); // comment out this line, you'll see compile error

    let ss3 = String::from("hello");
    let ss4 = ss3.clone(); // deep copy the heap data of the String, it's an expensive operation
    println!("ss3: {}, ss4: {}", ss3, ss4);

    /*
    The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, 
    so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from 
    being valid after we create the variable y. 
    */
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let arr = [1, 2, 3, 4, 5];
    let arr2 = arr;
    println!("arr = {:?}, arr2 = {:?}", arr, arr2);

    let c = String::from("hello");
    takes_ownership(c);     // c's value moves into the function...
    // println!("c: {}", c);   // ... and so is no longer valid here

    let z = 5;
    makes_copy(z);          // z would move into the function,
                            // but i32 is Copy, so it's okay to still
                            // use z afterward
    println!("z: {}", z);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}, s3: {}", s1, s3);

    let sss1 = String::from("hello");
    let (sss2, len) = calculate_length(sss1);
    println!("The length of '{}' is {}.", sss2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // let some_string = String::from("yours");
    // return some_string;
    return String::from("yours");
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}