fn main() {
    show_rules();
    scope_example();
    intro_to_strings();
    scope_and_the_heap();
    good_int_copy();
    bad_string_copy();
    good_string_copy();

    ownership_scope1();
    ownership_scope2();
    return_multiple_values();
}

fn show_rules() {
    println!("===============================================================");
    println!("Ownership Rules of Rust:");
    print!("1. Each value in Rust has a variable that’s called its owner.\n\
            2. There can only be one owner at a time.\n\
            3. When the owner goes out of scope, the value will be dropped.\n");
    println!("===============================================================");
}

fn scope_example() {
    // _s is not valid here, it’s not yet declared
    {
        // _s is not valid here, it’s not yet declared
        let _s = "hello";
        // _s is valid from this point forward
    }
    // this scope is now over, and _s is no longer valid
}

fn intro_to_strings() {
    // Declare: immutable &str (string literal/slice) on stack
    let _immutable_stack_string = "Hello";

    // Declare mutable String on heap
    let mut mutable_heap_string = String::from("Hello");
    // Appends an &str to a String
    mutable_heap_string.push_str(", world!");
    println!("{}", mutable_heap_string);
}

fn scope_and_the_heap() {
    // _s is not valid here, it’s not yet declared
    {
        let _s = String::from("hello");
        // _s is valid from this point forward
    }
    // this scope is now over, and _s is no longer valid
    // BUT the memory is automatically returned once the variable that owns it
    // goes out of scope. This is called the "Drop" trait.
}

fn good_int_copy() {
    let x = 5;
    let y = x;
    println!("y = {}", y);
    println!("x = {}", x);
    // Seems reasonable. Works fine. Let's try it with a String:
    // see bad_string_copy()
}

fn bad_string_copy() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {}", s2); // compiles fine up to here
    // println!("{}", s1); Error here!

    // Why? Because on the line let s2 = s1;, s2 replaces s1.
    // ie. The memory is copied, and then s1 is freed. This is called a "move".
    //      (Think shallow copy, but then s1's reference is invalidated)
    // To actually copy the String (deep copy), we need some more work.
    // See good_string_copy().
}

fn good_string_copy() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s2 = {}", s2); // s2 points to a new set of data (deep copy of s1)
    println!("s1 = {}", s1); // s1 still points to original set of data
    // all okay!

    // So why does good_int_copy() work?
    // It's because the integers are on the stack, and have a known size at
    // compile time so copies of the values are quick to make.
    // ie. integers have "copy" trait, which means original reference is still
    // valid even after a direct copy (with =).
    //
    // Rust won’t let us annotate a type with the Copy trait if the type,
    //   or any of its parts, has implemented the Drop trait. If the type needs
    //   something special to happen when the value goes out of scope and we add
    //   the Copy annotation to that type, we’ll get a compile time error.
    //
    // Other types with the copy trait:
    //     - all integers
    //     - all floats
    //     - bool
    //     - char
    //     - Tuples, but only if they contain types that are also Copy.
    //               ie. (i32, i32) is Copy, but (i32, String) is not.
}


//======================= Ownership Example 1 ==================================
fn ownership_scope1() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                                  // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                              // but i32 is Copy, so it’s okay to still
                              // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


//======================= Ownership Example 2 ==================================
fn ownership_scope2() {
    let _s1 = gives_ownership(); // gives_ownership moves its return val into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                               // takes_and_gives_back, which
                                               // also moves its return value
                                               // into s3

} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its return value
                                 // into the function that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                                 // some_string is returned and
                                                // moves out to the calling
                                                // function
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

//======================= Returning multiple values ============================
fn return_multiple_values() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    // This is basically just a hack to get around the fact that ownership of
    // s1 will be lost to calculate_length.

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) // return the length, but also s, to (in a very hacky way) give
    // back the String we stole from the function which called this.
}

// So, to summarize. This works, but is too much ceremony, and personally I
// think it seems rather hacky, thus the need for... references!
// (see ch4_references_and_borrowing)
