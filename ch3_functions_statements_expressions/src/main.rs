fn main() {
    println!("Hello, world!");
    another_function();
    function_with_a_parameter(5);
    function_with_two_parameters(5, 6);
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