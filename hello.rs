fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("x: {}, y: {}", x, y);

    let e: i32 = 2147483647;
    let f: i32 = -2147483648;

    let g: u64 = 18446744073709551615;
    let h: u64 = 0;

    println!("e: {}, f: {}, g: {}, h: {}", e, f, g, h);

    let pi: f64 = 3.141592653589793;
    println!("pi: {}", pi);

    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    let letter: char = 'A';
    println!("letter: {}", letter);
}