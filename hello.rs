fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers Array: {:?}", numbers);
    // let mix = [1, 2, 3, "Hello", true];
    // println!("Mixed Array: {:?}", mix);
    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    let human: (String, i32, bool) = ("John".to_string(), 30, true);
    println!("Human Tuple: {:?}", human);

    let person = ("Alice", 25, true);
    println!("Person Tuple: {:?}", person);

    let mixed = ("Hello", 42, false, [1, 2, 3]);
    println!("Mixed Tuple: {:?}", mixed);

    let slice_numbers: &[i32] = &[1, 2, 4, 5];
    println!("Slice Numbers: {:?}", slice_numbers);

    let animal_slices: &[&str] = &["Dog", "Cat", "Bird"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"IT".to_string(),
        &"Accounting".to_string(),
        &"Science".to_string(),
    ];
    println!("Book Slices: {:?}", book_slices);

    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold: {}", stone_cold);

    let string: String = String::from("Hello, World!");
    let slice: &str = &string[7..12];
    println!("slice is: {}", slice);
}
