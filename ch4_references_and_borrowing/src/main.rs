fn main() {
    show_rules();
    test_calculate_length();
    test_change();
    illegal_scope_too_many_mut_1();
    illegal_scope_too_many_mut_2();
    legal_scope_too_many_mut_1();
    legal_scope_too_many_mut_2();
    avoiding_dangle();
}

fn show_rules() {
    print!("================================================================\n\
            The Rules of References:\n\
            - At any given time, you can have either (but not both of) one\n   \
            mutable reference or any number of immutable references.\n\
            - References must always be valid.\n\
            ================================================================\n")
}


fn test_calculate_length() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // s1 not moved - yay!

}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn test_change() {
    // let immutable_str = String::from("Hello");
    // illegal_change(&immutable_str);

    let mut mutable_str = String::from("Hello");
    legal_change(&mut mutable_str);
    println!("{}", mutable_str);

}

/*
fn illegal_change(some_string: &String) {
    some_string.push_str(", world!"); // not mutable!
}
*/

fn legal_change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// Rust only allows one mutable reference to a particular piece of data in a
// particular scope, to prevent data races
fn illegal_scope_too_many_mut_1() {
    let mut s = String::from("Hello");
    let _r1 = &mut s; // all good
    // let _r2 = &mut s; Illegal
}

// Mixing mutable and immutable references is also not allowed
#[allow(unused_mut)] // ignore compiler warnings for the unused mutable
fn illegal_scope_too_many_mut_2() {
    let mut s = String::from("Hello");
    let _r1 = &s; // all good
    let _r2 = &s; // all good
    // let _r3 = &mut s; Illegal
}

fn legal_scope_too_many_mut_1() {
    let mut s = String::from("Hello");
    {
        let _r1 = &mut s;
    } // _r1 now out of scope, so we can now do:
    let _r2 = &mut s;
}

#[allow(unused_mut)] // ignore compiler warnings for the unused mutable
fn legal_scope_too_many_mut_2() {
    let mut s = String::from("Hello");
    let _r1 = &s;
    let _r2 = &s;
    // In this example we are wasting the mutability of s, but it works fine
    // and is allowed.
}

/*
fn illegal_dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
*/

// Just return the String directly.
fn avoiding_dangle() -> String {
    let s = String::from("Hello");

    s
}