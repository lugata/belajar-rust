fn main() {
    hello_world();
    tell_height(180);
    human_id("Alice", 30, 165.2);
    let x = {
        let price = 100;
        let qty = 5;
        price * qty
    };
    println!("The total is: {}", x);
    let sum = add(5, 10);
    println!("The sum is: {}", sum);

    let weight = 70.0;
    let height = 1.75;
    let bmi = BMI(weight, height);
    println!("The BMI is: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, World!");
}

fn tell_height(height: i32) {
    println!("The height is: {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("Name: {}, Age: {}, Height: {} cm", name, age, height);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn BMI(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}
