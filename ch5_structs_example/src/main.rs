fn main() {
    no_structs();
    using_tuple();
    using_structs_more_meaning();
    derived_traits();
}

//==============================================================================

fn no_structs() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.",
            area_no_structs(width1, height1));
}

fn area_no_structs(width: u32, height: u32) -> u32 {
    width * height
}

//==============================================================================

fn using_tuple() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.",
             area_using_tuple(rect1));
}

fn area_using_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//==============================================================================

struct Rectangle1 {
    width: u32,
    height: u32,
}

fn using_structs_more_meaning() {
    let rect1 = Rectangle1 { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.",
             area_using_structs_more_meaning(&rect1));
}

fn area_using_structs_more_meaning(rectangle: &Rectangle1) -> u32 {
    rectangle.width * rectangle.height
}

//==============================================================================
#[derive(Debug)] // enable pretty printing for the Rectangle2 type
struct Rectangle2 {
    width: u32,
    height: u32,
}

fn derived_traits() {
    let rect1 = Rectangle2 { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1); // simple pretty print
    println!("rect1 is {:#?}", rect1); // multi-line pretty print
}
