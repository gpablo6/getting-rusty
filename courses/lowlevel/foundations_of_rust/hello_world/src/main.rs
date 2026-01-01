use std::io;

fn main() {
    let mut my_string: String = String::new();

    io::stdin()
        .read_line(&mut my_string)
        .expect("You gave us a bad value");

    println!("You typed: {}", my_string);
}
