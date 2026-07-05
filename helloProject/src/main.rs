fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length is: {}, s1 is: {}", s1, len);
}

fn print_lost(s: &String) {
    println!("The lost string is: {}", &s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
