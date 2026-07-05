#![allow(warnings)]
fn main() {
    // let age: u8 = 16;
    // println!("The value of age is: {}", age);
    // if age >= 18 {
    //     println!("You can drive a car.");
    // } else if age >= 16 {
    //     println!("You can drive a car with a learner's permit.");
    // } else {
    //     println!("You can't drive a car.");
    // }

    // let number: i32 = 991;
    // if number % 4 == 0 {
    //     println!("The number is divisible by 4.");
    // } else if number % 3 == 0 {
    //     println!("The number is divisible by 3.");
    // } else if number % 2 == 0 {
    //     println!("The number is divisible by 2.");
    // } else {
    //     println!("The number is not divisible by 4, 3, or 2.");
    // }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
