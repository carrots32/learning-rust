// Note: String slice range indices must occur at valid UTF-8 character
// boundaries. If you attempt to create a string slice in the middle of a
// multibyte character, your program will exit with an error. For the purposes
// of introducing string slices, we are assuming ASCII only in this section;
// a more thorough discussion of UTF-8 handling is found in Chapter 8.


fn main() {
    test_first_word();
    intro_to_string_slices();
    test_better_first_word();
    test_even_better_first_word();
    slice_array();
}


// Returns the index of the first space in the given string, or if no spaces are
// found, return it's length
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Gets String as array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate with index
        // & needed because iter().enumerate() gives us a reference only
        if item == b' ' { // byte literal of space
            return i; // space found, return its index
        }
    }

    s.len() // no spaces found, return whole String's length
}

// Our goal is to use this to get and be able to use the first word of a String
fn test_first_word() {
    let mut s = String::from("Hello");
    let _word = first_word(&s); // _word = 5. looking good so far. We can use
    // this along with s to get the first word of s

    s.clear(); // s now equals ""

    // here, _word still has the value 5 (obviously), but s has changed, making
    // _word useless to us now. We really need a way to actually store a new
    // String that is just the first word.
}

// This get's even more difficult to deal with if we needed to get the second
// word: we'd need to track a starting and an ending index:
// fn second_word(s: &String) -> (usize, usize) { ...
// So let's try using String slices instead:

fn intro_to_string_slices() {
    let s = String::from("hello world");

    let _hello_part = &s[0..5];
    let _world_part = &s[6..11];

    // Alternatively:
    let _hello_part_inclusive_of_end = &s[0..=4];
    let _world_part_inclusive_of_end = &s[6..=10];

    // Alternatively:
    let _hello_part_starting_from_zero = &s[..4]; // don't need 0 at start
    let _world_part_ending_at_len_of_s = &s[6..]; // don't need len at end

    // Or for whole string:
    let _whole_slice = [..];
}

// Also would work for a second_word() function
fn better_first_word(s: &String) -> &str { // &str is 'string slice'
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // return s from 0 to index of first space
            // (not inclusive)
        }
    }
    &s[..] // No spaces found, return whole string.
}

#[allow(unused_mut)] // ignore compiler warnings for the unused mutable
fn test_better_first_word() {
    let mut s = String::from("hello world"); // immutable borrow on s starts

    let _word = better_first_word(&s);

    // s.clear();  // Compile error here - that's much better than only finding
    // out that your index var is useless when you try to use it
    // ^ attempted mutable borrow!!! But as we recall, you can't mutably borrow
    //                               something we have already borrowed!
    //                               (mutably or not)
} // immutable borrow on s ends

// Exact same code as better_first_word, but different signature: note that it
// takes an &str as the param rather than a &String
fn even_better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // return s from 0 to index of first space
            // (not inclusive)
        }
    }
    &s[..] // No spaces found, return whole string.
}

fn test_even_better_first_word() {
    let my_string = String::from("hello world");

    // first_word works on slices of Strings
    let word1 = even_better_first_word(&my_string[..]);

    // Note: It's not in the tutorial but it also works with regular Strings
    //   because &String can dereference to &str (because String implements
    //   Deref<Target=str>). This is called 'deref coercions'.
    //   Basically &A can coerce to &B if A implements Deref<B>.
    let word2 = even_better_first_word(&my_string);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word3 = even_better_first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word4 = even_better_first_word(my_string_literal);

    println!("The value of word1 is: '{}'", word1);
    println!("The value of word2 is: '{}'", word2);
    println!("The value of word3 is: '{}'", word3);
    println!("The value of word4 is: '{}'", word4);
}

fn slice_array() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("a = {:?}", a);
    println!("&a[1..3] = {:?}", slice);
}