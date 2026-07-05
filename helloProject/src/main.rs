fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    const Y: u16 = 10;
    println!("The value of Y is: {}", Y);
    println!("The value of PI is: {}", PI);
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );
}

const PI: f64 = 3.141592653589793;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
