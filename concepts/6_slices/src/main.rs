fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // same
    let slice = &s[0..2];
    {
        let slice = &s[..2];
        println!("slice: {}", slice);
    }
    println!("slice: {}", slice);

    let len = s.len();
    // same
    let slice2 = &s[3..len];
    {
        let slice2 = &s[3..];
        println!("slice2: {}", slice2);
    }
    println!("slice2: {}", slice2);

    // same
    let slice3 = &s[0..len];
    {
        let slice3 = &s[..];
        println!("slice3: {}", slice3);
    }
    println!("slice3: {}", slice3);

    #[allow(unused_mut)]
    let mut ss = String::from("hello world");
    let word = first_word(&ss); // immutable borrow
    // Recall from the borrowing rules that if we have an immutable reference to something,
    // we cannot also take a mutable reference.
    // ss.clear(); // mutable borrow // error
    println!("the first word: {}", word);

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice_a = &a[1..3];
    println!("{:?}", slice_a);
    assert_eq!(slice_a, [2, 3]);
    assert_eq!(slice_a, &[2, 3]);
}

// Defining a function to take a string slice instead of a reference to a String
// makes our API more general and useful without losing any functionality:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}