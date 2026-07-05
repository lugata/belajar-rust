fn main() {
    let mut _x = 5;
    println!("x is: {}", _x);
    let _r = &mut _x;
    println!("r is: {}", _r);
    *_r += 1;
    println!("r is: {}", _r);
    *_r += 3;
    println!("r is: {}", _r);
}
