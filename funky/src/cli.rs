use std::env;

pub fn io() {
    // I/O
    let args: Vec<String> = env::args().collect();
    let name = args[1].clone();
    println!("Name: {}", name);
    println!("Arguments: {:?}", args);

    if name == "naruto" {
        println!("Hello Hokage! Good Morning...");
    } else if name == "sasuke" {
        println!("Hello Uchiha! Good Morning...");
    } else {
        println!("Can't Detect any Ninja!")
    }
}