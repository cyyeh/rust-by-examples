/*
Printing is handled by a series of macros defined in std::fmt some of which include:

format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as format! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint!but a newline is appended.
*/
fn main() {
	// In general, the `{}` will be automatically replaced with any arguments
	println!("{} days", 31);

	// Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

	// optional arguments
	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	// named arguments
	println!("{subject} {verb} {object}",
			object="the lazy dog",
			subject="the quick brown fox",
			verb="jumps over");

	// special formatting can be specified after a `:`
	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

	// "     1": 5 white spaces and a "1"
	println!("{number:>width$}", number=1, width=6);

	// pad numbers with extra zeros. This will output "000001"
	println!("{number:0>width$}", number=1, width=6);

	#[allow(dead_code)] // suppress warning, #[warn(dead_code)] on by default
	struct Structure(i32);

	// println!("This struct `{}` whon't print...", Structure(3));

	// For Rust 1.58 and above, you can directly capture the argument from
	// surronding variable.
	let number: f64 = 1.0;
	let width: usize = 6;
	println!("{number:>width$}");
}