#![allow(warnings)]

use std::result;
fn main() {
    // let mut counter = 0;
    // let mut remaining = 10;

    // let result = loop {
    //     counter += 1;
    //     remaining -= 1;

    //     if remaining == 0 {
    //         break counter * 2;
    //     }
    //     if counter > 5 {
    //         break counter * 3;
    //     }
    // };
    // println!("Counter: {}", counter);
    // println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
        break;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    let b = ["a", "b", "c", "d", "e"];

    for element in a {
        println!("The value is: {}", element);
    }

    for element in b {
        println!("The value is: {}", element);
    }
}
