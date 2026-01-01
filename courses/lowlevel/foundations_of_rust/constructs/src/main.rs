struct Cat {
    name: String,
    age: u8,
}

impl Cat {
    // Your code here
    pub fn new(
        name: String,
        age: u8
    ) -> Self {
        Cat {
            name,
            age
        }
    }
}

fn run_main_cat() {
    let my_cat = Cat::new(String::from("Whiskers"), 3);
    println!("My cat's name is {} and she is {} years old.", my_cat.name, my_cat.age);
}

trait Animal {
    fn speak(&self);
}

struct Dog {
    name: String,
    age: u8,
}

// implement the trait!
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!")
    }
}

fn run_main_dog() {
    let my_dog = Dog {
        name: String::from("Buddy"),
        age: 5,
    };
    my_dog.speak();
}

fn main() {
    println!("Choose an option:");
    println!("1. Run cat example");
    println!("2. Run dog example");
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => run_main_cat(),
        "2" => run_main_dog(),
        _ => println!("Invalid choice"),
    }
}   