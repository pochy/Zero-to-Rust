fn main() {
    let name = String::from("Rust");

    borrow_name(&name);
    take_name(name);

    // Uncommenting this line causes a compile error because `name` moved into
    // `take_name`.
    // println!("{}", name);
}

fn borrow_name(value: &str) {
    println!("borrowed: {}", value);
}

fn take_name(value: String) {
    println!("owned: {}", value);
}
