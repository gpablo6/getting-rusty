use std::io;

fn middle_three(arr: &[i32; 5]) -> &[i32] {
    // Your code here
    let slice: &[i32] = &arr[1..4];
    return slice
}

fn run_main_middle_three() {
    let arr = [5, 10, 15, 20, 25];
    let result = middle_three(&arr);

    println!("{:?}", result); // Should print [10, 15, 20]
    assert_eq!(result, &[10, 15, 20]);
}

// === You implement this part ===
enum Message {
    // TODO: Add enum variants
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(u8, u8, u8)
}

fn process_message(msg: Message) -> String {
    // TODO: Match and return appropriate string
    let result: String = match msg {
        Message::Quit => String::from("Quit"),
        Message::Move {x, y} => format!("Moving to ({}, {})", x, y),
        Message::Write(msg) => format!("Text: {}", msg),
        Message::ChangeColor(r, g, b) => format!("Changing color to ({}, {}, {})", r, g, b)
    };
    return result;
}

fn run_main_process_message() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("{}", process_message(msg1)); // Should print "Quit"
    println!("{}", process_message(msg2)); // Should print "Moving to (10, 20)"
    println!("{}", process_message(msg3)); // Should print "Text: Hello, Rust!"
    println!("{}", process_message(msg4)); // Should print "Changing color to (255, 0, 0)"
}


fn main() {
    // print options to choose from
    println!("Choose an option:");
    println!("1. Run middle_three function");
    println!("2. Run process_message function");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => run_main_middle_three(),
        "2" => run_main_process_message(),
        _ => println!("Invalid choice"),
    }
}