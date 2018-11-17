use std::{thread, time::Duration}; // For sleep() in while_loop()
use std::io::{stdout, Write}; // for flush() without line break in while_loop()

fn main() {
    println!("\n=== Calling using_if_else(3); ===");
    using_if_else(3); // < 5
    println!("\n=== Calling using_if_else(6); ===");
    using_if_else(6); // > 5

    println!("\n=== Calling valid_bool(); ===");
    valid_bool();

    println!("\n=== Calling using_if_else_elseif(8); ===");
    using_if_else_elseif(8);
    println!("\n=== Calling using_if_else_elseif(6); ===");
    using_if_else_elseif(6);
    println!("\n=== Calling using_if_else_elseif(2); ===");
    using_if_else_elseif(2);
    println!("\n=== Calling using_if_else_elseif(7); ===");
    using_if_else_elseif(7);

    println!("\n=== Calling assigning_var_with_if_expr(); ===");
    assigning_var_with_if_expr();

    println!("\n=== Calling return_from_loop(); ===");
    return_from_loop();

    println!("\n=== Calling while_loop(); ===");
    while_loop();

    println!("\n=== Calling while_loop_iteration(); ===");
    while_loop_iteration();

    println!("\n=== Calling for_loop_iteration(); ===");
    for_loop_iteration();

    println!("\n=== Calling for_loop_range_with_rev(); ===");
    for_loop_range_with_rev();
}

fn using_if_else(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// Note that Rust needs an if condition to be a boolean type.
// It will not automatically cast say, an integer to a bool, like some other
// languages. As such, the following would generate a compile error:
/*
fn invalid_bool() {
    let number = 3;
    if number {
        println!("Number was 3!")
    }
}
*/

// This gets around the above
fn valid_bool() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn using_if_else_elseif(number: i32) {

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn assigning_var_with_if_expr() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    // Here we used the result of an if expression to assign a value using 'let'
    // However note we can't replace 6 with "six" because Rust needs to know at
    // compile time what type the number variable is going to be.
    println!("The value of number is: {}", number);
}

// Let's maybe not demonstrate this...
fn _infinite_loop() {
    loop {
        println!("again!");
    }
}

fn return_from_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // breaks and returns from statement;
        }
    };

    println!("Went through loop {} times. Result is {}.", counter, result);
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        print!("{}... ", number);
        stdout().flush().expect("Could not flush stdout"); // flush buf
        number -= 1;
        thread::sleep(Duration::from_secs(1)); // add a 1s delay
    }
    println!("LIFT-OFF!!!");
}

fn while_loop_iteration() {
    println!("While loop iteration:");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop_iteration() {
    println!("For loop iteration:");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_loop_range_with_rev() {
    for number in (1..4).rev() {
        print!("{}... ", number);
        stdout().flush().expect("Could not flush stdout"); // flush buf
        thread::sleep(Duration::from_secs(1)); // add a 1s delay
    }
    println!("LIFT-OFF!!!");
}