/*
Rust has a number of features that allow you to manage your code’s organization,
including which details are exposed, which details are private, and what names are in each scope in your programs.
These features, sometimes collectively referred to as the module system, include:
1. Packages: A Cargo feature that lets you build, test, and share crates
2. Crates: A tree of modules that produces a library or executable
3. Modules and use: Let you control the organization, scope, and privacy of paths
4. Paths: A way of naming an item, such as a struct, function, or module
*/

/*
Packages
A package is one or more crates that provide a set of functionality.
A package contains a Cargo.toml file that describes how to build those crates.
*/

/*
Crates
A crate can be a binary crate or a library crate. Binary crates are programs you can compile to an executable that
you can run, such as a command-line program or a server. They must have a function called main that defines
what happens when the executable runs. All the crates we’ve created so far have been binary crates.
Library crates don’t have a main function, and they don’t compile to an executable. They define functionality
intended to be shared with multiple projects. A package can contain at most one library crate.
It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

When we entered the command, Cargo created a Cargo.toml file, giving us a package. Looking at the contents of Cargo.toml,
there’s no mention of src/main.rs because Cargo follows a convention that src/main.rs is the crate root of a binary
crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs,
the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.
Cargo passes the crate root files to rustc to build the library or binary.
*/

/*
Modules
1. Start from the crate root:
    When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or
    src/main.rs for a binary crate).
2. Declaring modules: 
    In the crate root file, you can declare a new module named, say, “garden”, with mod garden;.
    The compiler will look for the code inside the module in these places:
        Inline, directly following mod garden, within curly brackets instead of the semicolon
        In the file src/garden.rs
        In the file src/garden/mod.rs
3. Declaring submodules:
    In any file other than the crate root that’s being compiled as part of the crate (for example, src/garden.rs),
    you can declare submodules (for example, mod vegetables;). The compiler will look for the code inside submodules
    in these places within a directory named for the parent module:
        Inline, directly following mod vegetables, within curly brackets instead of the semicolon
        In the file src/garden/vegetables.rs
        In the file src/garden/vegetables/mod.rs
4. Paths to code in modules:
    Once a module is being compiled as part of your crate, you can refer to code in that module (for example,
    an Asparagus type in the garden vegetables module) from anywhere else in this crate by using the path
    crate::garden::vegetables::Asparagus as long as the privacy rules allow.
5. Private vs public:
    Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod
    instead of mod. To make items within a public module public as well, use pub before their declarations.
6. The use keyword:
    Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths.
    In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with
    use crate::garden::vegetables::Asparagus; and then only need to write Asparagus to make use of that type in the scope.
*/

/*
Paths
A path can take two forms:
1. An absolute path starts from a crate root by using a crate name (for code from an external crate) or
    a literal crate (for code from the current crate).
2. A relative path starts from the current module and uses self, super, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
*/

fn main() {
    println!("Hello, world!");
}
