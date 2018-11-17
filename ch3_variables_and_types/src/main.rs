const MAX_POINTS: u32 = 100_000; // Can add '_' to num literals for readability


fn main() {
    println!("\n=== Using the Constant ===");
    println!("Max points = {}", MAX_POINTS);
    
    println!("\n=== Mutability ===");
    mutability();
    
    println!("\n=== Shadowing ===");
    shadowing();
    
    println!("\n=== Mutating Types (illegal) ===");
    mutate_type();

    println!("\n=== Data Types (see source code) ===");
    data_types();
    compound_data_types();
    numeric_operations();
}

fn mutability() {
    let mut x = 5; // mut needed to make it mutable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is now: {}", x);
}


fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn mutate_type() {
    let spaces = "  ";
    println!("spaces = \"{}\"", spaces);
    let spaces = spaces.len();
    println!("spaces = \"{}\" \
            (shadowed - can't actually change type of mutated variable)",
             spaces);

    // This is fine because we are just shadowing the first with the second
    // However the following is illegal:
    /*
     let mut spaces = "  ";
     spaces = spaces.len();
     */
}

#[allow(unused_variables)] // ignore compiler warnings for unused_variables
fn data_types() {
    // Integer Types
    let signed_8_bit: i8 = 0;
    let unsigned_8_bit: u8 = 0;
    let signed_16_bit: i16 = 0;
    let unsigned_16_bit: u16 = 0;
    let signed_32_bit: i32 = 0; // default
    let unsigned_32_bit: u32 = 0;
    let signed_64_bit: i64 = 0;
    let unsigned_64_bit: u64 = 0;
    let signed_128_bit: i128 = 0;
    let unsigned_128_bit: u128 = 0;
    let signed_32or64_bit: isize = 0;   // architecture-dependent
    let unsigned_32or64_bit: usize = 0; // architecture-dependent

    // Integer Literals
    let decimal_type = 98_222;            // defaults to i32
    let hex_type = 0xff;                  // defaults to i32
    let octal_type = 0o77;                // defaults to i32
    let binary_type = 0b1111_0000;        // defaults to i32
    let byte_type = b'A';                 // defaults to u8 (only)
    let non_byte_type_with_suffix = 57u8; // suffix forces type

    // Floating Point Types
    let fp_64 = 2.0; // f64 (default)
    let fp_32: f32 = 3.0; // f32

    // Boolean Types
    let t = true;
    let f: bool = false; // with explicit type annotation (not needed)

    // Character types
    let char_lower = 'z';
    let char_upper = 'Z';
    let emoji = '😻'; // Note unicode scalar, so more than just ASCII!
    // Range: U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
}

#[allow(unused_variables)] // ignore compiler warnings for unused_variables
fn compound_data_types() {
    // Tuple Type
    let tup_explicit: (i32, f64, u8) = (500, 6.4, 1);
    let tup_implicit = (500, 6.4, 1);

    // now to get an individual value (destructuring):
    let (x, y, z) = tup_implicit; // now x = 500, y = 6.4, z = 1

    // Alternatively, use a period (.)
    let five_hundred = tup_implicit.0;
    let six_point_four = tup_implicit.1;
    let one = tup_implicit.2;

    // Array Type
    let arr_explicit: [u8;5] = [1, 2, 3, 4, 5]; // Limited to 5 elements [u8;5]
    let arr_implicit = [1, 2, 3, 4, 5];  // Limited to 5 elements [i32;5]
    let months = ["January", "February", "March", "April", "May",
            "June", "July", "August", "September", "October", "November",
            "December"]; // [&str; 12]

    // Array Access
    let first_num = arr_implicit[0]; // i32
    let second_month = months[1];   // &str
    // let thirteenth_month = months[12]: runtime error! (not compile)
}

#[allow(unused_variables)] // ignore compiler warnings for unused_variables
fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let float_quotient = 56.7 / 32.2; // = 1.7608695652173911
    let int_quotient2 = 5/2; // = 2

    // remainder
    let remainder = 43 % 5;
}
