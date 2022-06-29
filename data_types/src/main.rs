fn main() {
    /*
    Scalar types
    */
    // In cases when many types are possible, we must add a type annotation
    #[allow(unused_variables)]
    let guess: u32 = "42".parse().expect("Not a number!");

    /*
    integer types
    length  signed  unsigned
    8-bit   i8      u8
    16-bit  i16     u16
    32-bit  i32     u32
    64-bit  i64     u64
    128-bit i128    u128
    arch    isize   usize

    integer literals
    decimal       98_222  "_" as a visual separator to make the number easier to read
    hex           0xff
    octal         0o77
    binary        0b1111_0000
    byte(u8 only) b'A'
    */

    // floating point types, no unsigned
    #[allow(unused_variables)]
    let x = 2.0; // f64
    #[allow(unused_variables)]
    let y: f32 = 3.0; // f32

    #[allow(unused_variables)]
    let floored = 2 / 3; // results in 0

    // boolean types
    #[allow(unused_variables)]
    let t = true;
    #[allow(unused_variables)]
    let f: bool = false;

    // character type, 4-bytes, Unicode
    #[allow(unused_variables)]
    let c = 'z';

    /*
    Compound types
    */
    // tuple types
    // a variety of types, fixed length
    // The tuple without any values, (), is a special type that has only one value, also written ().
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    #[allow(unused_variables)]
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The first value is: {}", tup.0);
    println!("The value of tuple is: {:?}", tup);

    // array types
    // must have the same type, fixed length
    // Arrays are useful when you want your data allocated on the stack rather than the heap
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("The value of a is: {:?}", a);
    println!("The value of b is: {:?}", b);
    println!("The value of c is: {:?}", c);
    println!("The first value of a is: {}", a[0]);
}
