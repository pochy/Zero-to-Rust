use std::mem::size_of;

fn print_size<T>(name: &str) {
    println!("{name:<16} {} bytes", size_of::<T>());
}

fn main() {
    print_size::<i32>("i32");
    print_size::<usize>("usize");
    print_size::<String>("String");
    print_size::<Vec<u8>>("Vec<u8>");
    print_size::<&str>("&str");
    print_size::<&[u8]>("&[u8]");
    print_size::<Box<i32>>("Box<i32>");

    let text = String::from("Rust computer systems");
    let bytes = vec![1_u8, 2, 3, 4, 5];

    println!("String len       {} bytes", text.len());
    println!("String capacity  {} bytes", text.capacity());
    println!("Vec len          {} items", bytes.len());
    println!("Vec capacity     {} items", bytes.capacity());
}
