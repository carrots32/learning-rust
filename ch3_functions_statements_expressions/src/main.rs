fn main() {
    println!("Hello, world!");
    another_function();
    function_with_a_parameter(5);
    function_with_two_parameters(5, 6);
    statement();
    expression();
    use_return_value();
    test_plus_one();
}

fn another_function() {
    println!("Another function.");
}

fn function_with_a_parameter(x: i32) {
    println!("The value of x is {}", x)
}

fn function_with_two_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statement() {
    let _y = 6; // A statement (but see expression() for side note)
    // Statements return nothing. As such, the following would err:
    // let x = (let y = 6);
    // ie. You can't do x = y = 6 and have x and y both equate to 6.
    // Side note, underscore used to ignore unused_variable warnings
}

fn expression() {
    let _x  = 5;
    //       ^ The number 5 here is actually an expression, that equates to 5.

    // Some other expressions:
    //   - Calling a function
    //   - Calling a macro
    //   - The block we use to create new scopes: {}. Example below:
    let _y = {
        let x  = 5;
        x + 1
    };
    // Note no semicolon on last line.
    // This is essentially the return value of the expression, in this case, 4.
}

fn function_with_return_value_of_5() -> i32 {
    5 // Like saying 'return 5;' in other languages
}

fn use_return_value() {
    let x = function_with_return_value_of_5();
    println!("The value of x is: {}", x);
}

fn test_plus_one() {
    let x = plus_one(5);
    println!("The value of x (plus_one(5)) is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
} // Note it would give compile error if ending with semicolon

