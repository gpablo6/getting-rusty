fn take_ownership(s: String) -> String {
    println!("Inside function: {}", s);
    // s is dropped here
    s
}

fn run_main_ownership() {
    let s1 = String::from("Hello, Rust!");
    let s2 = take_ownership(s1);
    println!("Back in main: {}", s2);
    // s1 is no longer valid here
}

fn borrow_string(msg: &String) {
    println!("Inside function: {}", msg);
}

fn run_main_borrowing() {
    let s1 = String::from("Hello, Borrowing!");
    borrow_string(&s1);
    println!("Back in main: {}", s1);
    // s1 is still valid here
}

fn main() {
    println!("Choose an option:");
    println!("1. Run ownership example");
    println!("2. Run borrowing example");
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => run_main_ownership(),
        "2" => run_main_borrowing(),
        _ => println!("Invalid choice"),
    }
}
